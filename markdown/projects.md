# > tree /data/Projects

---

There isn't a real order to this, neither they are all single projects, neither there's all my projects (like this website) and believe me it's better like this. 
It's just a bunch of stuff I've done over the years, some of them are still in progress, some are just for fun, and some are just experiments.

The rest of my projects can be found on my [GitHub](https://github.com/JLsquare), or lost somewhere maybe who knows.

---

## [QTQuickDetect](https://github.com/JLsquare/qtquickdetect)

_python - pyqt - machine learning_

One of my biggest project, QTQuickDetect is a user-friendly application for educational purposes, designed to test and demonstrate the capabilities of deep learning models in object detection, segmentation, classification, and pose estimation on various media types.  

It offers an intuitive platform for students to engage with AI technologies, providing detailed results, customizable settings, and support for GPU acceleration.

---

## torch experiments

_cpp - cuda - pytorch - machine learning_

I'm experimenting with PyTorch and the math behind machine learning, but I prefer to go deep when I can.
So I'm trying to implement some custom CUDA extensions for PyTorch in C++, with ideas I have in mind, like a positional sparse linear layer (tiny world graph), who is extremely slow if we use only python and PyTorch.

If I write things on this website, it will probably be about all this stuff.

---

## YACP, Yet Another Chat Protocol

_rust - networking - systems programming_

It's a chat protocol I had in mind for a while, and I wanted to implement it in Rust. 

It's still a work in progress, but the idea is to have a overly secure and anonymous chat protocol inspired by the Tor network and IRC. 

---

## [Voxel Engine](https://github.com/JLsquare/voxel-dda-rs/tree/main)

_rust - computer graphics_

This was a project I started because I always was fascinated by voxels and procedural generation, and I wanted to learn more about computer graphics.
For now, it's just a renderer on CPU who implements the [fast voxel traversal algorithm](https://github.com/cgyurgyik/fast-voxel-traversal-algorithm/blob/master/overview/FastVoxelTraversalOverview.md), and a basic procedural terrain generation with perlin noise.

---

## Discord LLM

_discordjs - fine-tuning - machine learning - rust_

I wanted to try some fine-tuning on a GPT-2 model (at the time) to generate messages in a discord bot, with discord messages as the training data, what a shitty idea. 
So first I made a [messages scrapper in rust](https://github.com/JLsquare/discord-scraper-rs), who ip-banned my whole student residence and my university's network temporary, sowwy 👉👈. 
It was a junky stupid LLM but I'm currently doing a better and cleaner version.

---

## [place-rs](https://github.com/JLsquare/place-rs)

_rust - web - networking_

After botting the real one, I've made an overly optimized and lightweight Reddit's r/place clone in Rust, using websockets, full memory caching, sqlite database for persistence, and a lot of other cool stuff. 
It's fully dockerized and can run on a 1GB Raspberry Pi without any problem, the only bottleneck is NGINX or the number of websockets connections. 
There is a authentication system, but any bot can bypass it, so bot detection is something to explore in the future.

---

## [voxplace](https://github.com/JLsquare/voxplace)

_nuxt - web - three.js - rust - networking_

I wanted to try to make an entire social network full-stack application to see if I could do it.
It's a social network centered around Voxel Art, with a 3D voxel editor, a gallery, a insta/tweet-like feed, profiles, comments, likes, and all that jazz.
And there is also "places" where you can draw on a 3D canvas with other users, like Reddit's r/place, but in 3D.
The backend is in Rust, with a websocket server, and the frontend is in Nuxt.js with three.js for the 3D stuff.
It works, but it was never meant to be a real thing, **if you are interested to continue this project, feel free to reach out.**

---

## [Random school projects](https://github.com/JLsquare?tab=repositories)

_java - python - android - web - ..._

Just a bunch of stuff I had to do for school. There's some [Reed Salomon thing](https://github.com/JLsquare/reed-salomon-qr-code/blob/main/Projet_Reed-Salomon_QR.ipynb), a bit of basic [machine learning](https://github.com/JLsquare/R4.A.13-ia-cnn/tree/main), 
and a couple of todo apps ([Android](https://github.com/JLsquare/R4.A.11-todoapp) and [Vue.js](https://github.com/JLsquare/R4.A.10-vue-todo-app)) because apparently that's what you do when learning app dev. 
Oh, and a [sports tracker](https://github.com/JLsquare/R3.01-sport-track) too.

---

## [Bikes of Nantes](https://github.com/JLsquare/velo-nantes-javafx/tree/main)

_java - javafx - sql - ..._

Please don't go there. It's a JavaFX app I made for a school project, it's not bad, but it's Java. 
It allow you to analyze the data of the bikes in Nantes, with a map, graphs... 
With a MySQL/MariaDB database do add / edit data with a simple DAO pattern.

---

## A ton of Unity projects

_c# - unity - game dev_

That's where I started back in 2015. A bunch of small games / experimentation's, for game-jams, tiny discord "studios", or just for fun.
I think that's where I learned the most about the fundamentals of programming, networking, OOP, procedural generation, GPU programming, and all that jazz.
It helped me a lot with my studies, even though I don't do much game dev anymore, and also allowed me to meet a lot of cool people.

---

## And a lot of other stuff

A lot of things didn't make it to my GitHub, either because they were too small, too bad, or too personal, or i forgor 💀. 
I've done a lot of small scripts, tools, and experiments over the years, and I'm always looking for new ideas and projects to work on. 
If you have any suggestions or want to collaborate on something, feel free to reach out!