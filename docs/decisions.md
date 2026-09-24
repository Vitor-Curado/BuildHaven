# Engineering decisions

## Programming languages

Originally, the project was written in `python`, and Python isn't necessarily RAM intensive, but `gunicorn` is. Not really, but in the low-resouce environment that I am, every MB of RAM matters, and that was decisive in my need to rewrite to `Rust`. The language seemed new, fascinating and hot to me with all that it promises:

* type safety 
* performance comparable to `C`
* memory safety

Very early in development I could not for the life of me ship it in `Python` because of `gunicorn`, and when I shipped the prototype in `Rust` it worked. Simple as that.

## Database

 Postgres is *the deal* when it comes to DBs. It is the default option when it comes to most situations in lower scale projects, it can do the job of other technologies, such as `redis`. Easiest decision of my life.

## Operating system (prod and dev)

I have migrated from Windows to Fedora Linux in... so long ago, and I have been enjoying it thoroughly ever since. No particular reason

## Client Side Rendering (subject to change)

This isn't a minor decision because there's a whole story behind it. In the early stages, I remember being electric about this Rust framework which promised to be faster and more efficient than JavaScript itself, at the time I was salivating to making my website as JavaScript-free as possible. The framework I'm talking about is called `leptos`; it compiles directly into `Web Assembly` and that was super attractive to me.

The problem was when I faced reality, I realised the thing (at least when I tried it) wasn't minifying, fingerprinting, merging the static assets, and that was such a huge turn off for me that I left CSR entirely.

I have been dabbling with `TypeScript` and `React` because my understanding is that it is an `industry standard` and the difference felt night and day: cache busting, filer merging, minification, everything and I didn't even have to whine. My main problem with CSR as it currently stands is that

## SQLx or SeaORM 

## Continuous Integration, Continuous Deployment (CI/CD)

My original choice of CI/CD was `Woodpecker CI` coming from my obsession to have it as lean and mean as possible. In order to do that madness, not wanting to pay for another server instance, I was using my laptop with a woodpecker-agent and woodpecker-server, using Cloudfare to tunnel it to my laptop, but for some reason the both daemons weren't working. They needed babysitting that I just ran out of gas and I stopped paying attention. I have been meaning to experiment with `GH Actions`, but to be very honest with you, this project doesn't need CI/CD. The development is slow, and I can compile it in ARM/x86 and send it to prod, it's not like it's going to be the end of the world. Eventually, I will return and setup GH Actions, but it's a low priority for me.  

## Minor decisions

* **Reverse proxy:** It started with `Nginx`, and I have been rocking the initial config ever since. Haven't felt the need to change it in... so long, actually.
* **Orchestration:** There are usually two options, `systemd` (.service file) and `compose` (.yml file). I opted for `systemd` simply out of inertia; the one I'm most familiar with. It works, `podman` literally generated the .service file for me, and I don't need to check it ever.  
* **SSR templating engine:** Since we're in `Rust`, I ended up with `Askama`. No particular reason. Works flawlessly, feels like Jinja2 from Python.
* **Containerisation technology:** `Podman` out of inertia, the Fedora/RHEL/RPM native pod manager that *comes out of the box*, podman. Daemon-less, it's pretty good
* **Runtime image:** I tried `distroless` at first because I felt edgy and cool and sweaty tryhard about it shaving even the kernel, but I eventually felt the kernel was necessary for debugging, so I returned to `Debian's bookworm-slim` and have never looked back
* **Version Control System:** GitHub, I hope I don't need to explain this. Alongside with it, I use GitHub Container Registry a lot which has pretty good plans for public repositories
* **Cloud provider:** Initially DigitalOcean for its simplicity and ease of use, migrating to AWS since it no longer overwhelms me
* **Production server architecture:** Initially, I used x86 simply because there was no other option. In AWS, though, they offer ARM64 casually, and out of curiosity I decided to give it a try given ARM is a liiittle bit better than x86 when it comes to production servers. For me though it was more for the experience of it. Sometimes do I regret my decisions? Plenty, but it felt bearable enough not to whine my way back to x86. Because I do not have the dedicated hardware for it, though, it takes actual more than 10 minutes to build the image and ship it to GHCR
* **Version control system:** Fresh out of college, `Git` and `GitHub` are what one would expect from a junior. The ecosystem around it, for me `GitHub Container Registry` (GHCR), is what made me stick to it. I'm not saying others are bad because I don't know about other options, but sending my stuff to 