# r/place vision transformer

---

The last 10 days I finally tried implementing an idea I got from a long time, an AI that simulates a r/place player.  

I may have underestimated some parts, but I learned a lot in this project, from the dataset, the model(s) himself, and the post processing of AI output.

I will not enter in the details of the code here, or give quantitative metrics, I just wanted to yap about it.

---

## The dataset

It was a way bigger challenge than I thought.

The data we got from reddit is a big CSV file of +160 millions lines of each pixel change with the color, the coordinates, the timestamp, and the user (anonymized), and it's not in the right order.

And the goal is to take a random idx from 0 and 160 million, and reconstruct the canvas at the timestamp from the pixel change n° idx.

The naive approach is to each iteration, take the idx, take the timestamp at the row, and go through all the rows to reconstruct the canvas from scratch. It is obviously not a viable option.

So I took a different approach, there are obviously better approaches, but this one worked for me :
- I first inserted all the data in a sqlite database, can't use the timestamp as a primary key because there are duplicates, so no primary key for now. The color was the hex value, so I converted it to an int between 0 and 31 from the r/place palette.
- I created another sqlite database with the same schema, but with an id as primary key, and I inserted all the data in the right order.
- Then I divided the data in tiny database partitions of 1 million rows, still in the right order. (160 databases)
- And for each partition, I processed the final canvas, and saved it in a file.

All this implemented in a rust tool, it process the data in ~1h on my machine.

Now to get the canvas at an idx, I take the partition n° idx // 1 million, and I load the canvas from the file, and I apply the last changes for each x, y before the timestamp of the idx.

It was way faster, and I could run a batch of 192 at 4 it/s so ~800 canvas/s.

Now we got the input data, but we still need the target data. I have two datasets for two different models we will see later.

For the first one, the target is just the next color at the center of a canvas patch of 33x33, and for the second one, the target is for each pixel of the patch, when will it be changed in the future, normalized to an hour, so for more than 1 hour, it's just 1.0.

So it's just some different sql queries to get the target data, and process it in the python dataset class.

---

## The model(s)

After a lot of tries, I ended up with an architecture combining two models, a model to predict the next color at the center of a patch, and a model to predict the time before the next change for each pixel of the patch.

![Model architecture](/static/rplace-vision-transformer/1.svg)

The values for the d_ff, d_model, n_heads, n_layers, are from the paper [An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale](https://arxiv.org/abs/2010.11929)

I train each model separately, and then I combine them in the inference. I could maybe post train them together, but I didn't try yet. 

In fact, I didn't even trained the models on more than 1 epoch on more than 4 percent of the data, I prefer to enhance the dataset and the model before putting it in the oven, but I already got some interesting results.

Looking at the inference code, here's how the two models actually work together:
- The time model (RPlaceTimeTransformer) is used to predict which pixel in the current view is most likely to change next. It doesn't actually predict the time, but rather a probability or likelihood of change for each pixel. The pixel with the highest likelihood (plus some randomness) is chosen as the target for the color change.
- Once a pixel is chosen, the color model (RPlaceColorTransformer) is used to predict the new color for that pixel. It takes a view centered on the chosen pixel and outputs a probability distribution over the 32 colors in the r/place palette.
- The final color is chosen based on the color model's prediction, with some additional processing to avoid choosing the current color and to apply weights to certain colors (like reducing the probability of black).

This combination allows the system to both decide where to place a pixel and what color that pixel should be. The "time" model is really more about predicting the location of the next change, while the color model determines what that change should be.

Despite the limited training so far, this dual approach seems to capture some of the complex behaviors seen in r/place, like the formation of structures and the spread of colors. 

However, there's still a lot of room for improvement, especially in terms of consistency and long-term pattern formation.

---

## The post processing

The problem with this project is that the users are not the same, each users have a different goal, a different style, and a different place in the canvas. The AI for now is just a mix of all the users, from bots, the void, the flags, and the art.

It's like reducing the whole canvas to something like a 64x64 canvas, it would be pure chaos, not enough space for everyone ideas.

So for now, I'm just trying to do a little post processing on the model output, I tried the following:
- Multiply the color prediction by a weight map, with for example the black color being 0.75, to reduce the void.
- If the pixel is already the right color, take the second predicted color.
- Don't take the most probable color, but a random from the probabilities, with a temperature to control the randomness.

These post-processing steps aim to introduce variety and control into the model's output, helping to mitigate the "average behavior" problem and simulate more diverse user actions. The weight map helps reduce unwanted behaviors like excessive void creation, while choosing alternative colors and introducing randomness helps create more interesting and varied patterns.

Despite these efforts, balancing the various competing "user intentions" remains a significant challenge. The current approach is a starting point, but there's still room for more sophisticated strategies to better capture the diverse and sometimes conflicting goals of r/place participants.

---

## The results  

### 1.9 million parameters x 2, 1.5 million iterations model, 128x128, 17x17 patch (289 context size)

![Results](/static/rplace-vision-transformer/2.gif)

### 85 million parameters x 2, 3 million iterations model, 256x256, 17x17 patch (289 context size)

![Results](/static/rplace-vision-transformer/3.gif)

---

## The future

I've got several ideas to improve this project:

- Divide some layers into "experts" based on user classification or canvas zones. This could help the model better capture diverse user behaviors.
- Train a smaller model, as I noticed better pattern recognition with the 1.9 million parameter version compared to larger ones.
- Extended training obviously.
- Implement a level-of-detail (LOD) system using CNNs. This would involve embedding a larger view while maintaining a detailed per-pixel perspective, potentially improving both local and global coherence.

There's still much room for improvement, it's only the start, the "gpt-2" of this approach. I'm committed to refining this project further and will share updates once I'm satisfied with the progress.

I welcome any suggestions, collaborations, or discussions about the project. Feel free to reach out if you have ideas or want to contribute.

The code will be available on my GitHub soon, and I'll provide the link here when it's ready.