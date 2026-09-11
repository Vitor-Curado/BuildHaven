# What I have learned by programming this website

This project is, fundamentally, a lot of leg work. It made me change my mind countless times, face concepts I didn't come close in college, it boosted my confidence in someways, but also increased my impostor syndrome in others. I had periods of frenzy adding features for the sake of it, tokenism, then periods of retracting, but I never truly gave up. 

Development has been slow because... I have been enamored more with the frenzy of implementing for the sake of implementing, blitzing, exploring software concepts and using AI to achieve things that I realised I built myself a ton of crap I don't use, and my codebase is in need of a cleanup, testing, not more features.

## How it started

I started this project first in Python. In the process of trying to ship it, and inevitably having to rewrite it in Rust, I started thinking I would program things from scratch, as few dependencies as possible. Slowly, over the course of weeks to months, I came in contact that there's always a dependency, and with control comes responsibility. 

I remember to this day how it started, and it was with the thought "Fuck it, I haven't programmed anything meaningful during my program, let me ask ChatGPT what are the leanest, fastest tech stack that I could program with." At the time, I remember to be obsessed with raw speed, and I also had no idea how the speed of each and every part mattered. I remember the stack it gave me was:

(Everything Rust, because I explicitly asked for it)
* Askama (templating engine)
* SQLx ()
* Tokio (OFC, async runtime)

That's all that I knew. Everything was so bright; I remember not knowing what SSR was, nor CI/CD. These things were as alien to me as fossils in a museum. I remember