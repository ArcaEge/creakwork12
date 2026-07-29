# creakwork12

turn your framework 12 into a creaky wooden door, basically [this](https://github.com/samhenrigold/LidAngleSensor) (that was also the inspiration too) but ported to a fw12. it may or may not work on other laptops too, I haven't tested it. the filtering is optimised for framework 12's hinge stiffness.

## demo

https://github.com/user-attachments/assets/df616876-cd78-4019-ae8a-5441bb88333c

## features

- make your hinge go creaky
- tested on framework 12, might work on other laptops too
- ✨ effects ✨
  - pitch depends on position of hinge
  - volume depends on how fast you adjust it
- have fun

## why

funny

## how to

there is an executable in the releases tab. download it, do the usual `chmod +x` and run. enjoy. note: this was built on fedora, so your mileage may vary on other distros - there might be dynamic linking issues.

or git clone and do `cargo run`. you'll have to install `libiio-devel` and `alsa-lib-devel` first though if you do that (or `libasound2-dev` and `libalsa-dev` on debian/ubuntu)
