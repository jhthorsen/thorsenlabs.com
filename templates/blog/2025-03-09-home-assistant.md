---
title: Home Assistant
---

[Home Assistant](https://www.home-assistant.io) is an amazing project, and I can see how it's the [most popular](https://octoverse.github.com/2022/state-of-open-source) open source project. Many YouTubers say that Home Assistant is not for "common people", but I'm not sure if I agree. Yes, you have to have some tech knowledge, but I'm guessing you already do, if you're into home automation.

The only way I think you could do without Home Assistant, would be if you for example only have [HomeKit](https://www.apple.com/home-app/) supported devices. In my case however, I have the following apps on my phone to setup and manage my devices:

* [Apple Home](https://apps.apple.com/us/app/home/id1110145103) - Apple TV, HomePod
* [Eolia](https://apps.apple.com/app/id1261216665) - Panasonic air condition
* [LibreLink](https://apps.apple.com/app/id1449296861) - Blood glucose monitoring
* [Meross](https://apps.apple.com/app/id1260842951) - Smart outlets, lights
* [MyDyson](https://apps.apple.com/app/id993135524) - Heater/cooler
* [Nature Home](https://apps.apple.com/app/id1193531669) - Universal remote, sensors and energy
* [Nocria](https://apps.apple.com/app/id1577460780) - Fujitsu aircon
* [Qingping+](https://apps.apple.com/app/id1344636968) - Air quality monitor
* [Reolink](https://apps.apple.com/app/id995927563) - Camera
* [Sesame](https://apps.apple.com/app/id1532692301) - Smart lock
* [Smart Life](https://apps.apple.com/app/id1115101477) - Curtain, lights, sensors
* [Tapo](https://apps.apple.com/app/id1472718009) - Camera
* [VeSync](https://apps.apple.com/app/id1289575311) - Humifier

To sum it up: It's a mess and it's really hard to remember which app is for what. Therefore I'm very glad that I only use those apps for initial setup, and then I control all the devices through [Home Assistant](https://www.home-assistant.io). It's truly wonderful to have all the devices organized in one app. I do have the most important devices linked to Apple Home as well though, since it's nice to have it all integrated with [Siri](https://en.wikipedia.org/wiki/Siri) and it's very convenient to have the cameras pop up on the [Apple TV](https://www.apple.com/tv-home/).

## Custom devices with MQTT

I just recently realized that I can easily use the [MQTT integration](https://www.home-assistant.io/integrations/mqtt/) to create my own custom devices and entities. I known there's a "System monitor" integration, but just to get started, I wrote a bash script that pushes ping, temperature and system usage using [mosquitto_pub](https://github.com/eclipse-mosquitto/mosquitto) to the MQTT integration. It was a lot of fun seeing how easy it was to use the integration.

## Glucose control

All of the above is mostly just for fun, but the number one integration I use is the [LibreLink integration](https://github.com/gillesvs/librelink) to monitor my blood glucose level. The official iPhone app from Libre is... awful: You can't pan or zoom the graphs and the notifications is not helpful at all. I started using [Sweet Dreams](https://apps.apple.com/us/app/sweet-dreams-sugar-tracker/id1644428422) some months ago. It has a much nicer interface and the notifications are very helpful. However, I find myself getting irritated with how many notifications I get, so I've now replaced it with notifications pushed from Home Assistant instead. This is perfect for two reasons: I'm in complete control of the rules and I can get the values in both mmol/L and mg/dL.

## Summary

I started out using [Homebrigde](https://homebridge.io/), since I liked the simplicity and the tight integration with HomeKit, but I soon realized that Apple Home simply does not allow you to set up automations the way [Home Assistant](https://www.home-assistant.io) allow you to. Starting out with Home Assistant was a bit painful though, but that's mainly because the Home Assistant I started out with is no where near as user friendly as the Home Assistant of 2025. It could of course be because I've gotten better at managing it, but it most certainly also have to do with all the things you can now do using the web interface, instead of fiddling around with [YAML config](https://en.wikipedia.org/wiki/YAML) files directly on the server,

I'm very excited to see where Home Assistant is heading, and I hope that the device manufactures makes it easier to integrate new devices, instead of locking down API's.
