---
title: Home Assistant
status: draft
---

Home Assistant is an amazing project, and I can see how it's the most popular open source project. Many YouTubers say that Home Assistant is not for "common people", but I'm not sure if I agree. Yes, you have to have some tech knowledge, but I'm guessing you already do, if you're into home automation.

The only way I think you could do without Home Assistant, would be if you for example only have HomeKit supported devices. In my case how ever, I have the following apps on my phone to setup and manage my devices:

* Apple Home - Apple TV, HomePod
* Eolia - Panasonic aircon
* LibreLink - Blood glucose monitoring
* Meross - Smart outlets, lights
* MyDyson - Heater/cooler
* Nature Home - Universal remote, sensors and energy
* Nocria - Fujitsu aircon
* Qingping+ - Air quality monitor
* Reolink - Camera
* Sesame - Smart lock
* Smart Life - Curtain, lights, sensors
* Tapo - Camera
* VeSync - Humifier

To sum it up: It's a mess and it's really hard to remember which app is for what. Therefore I'm very glad that I only use those apps for initial setup, and afterwards I control all the devices through Home Assistant. It's truly wonderful to have all the devices organized in one app. I do have the most important devices linked to Apple Home as well, since it's nice to have it all integrated with Siri and it's very convenient to have the cameras pop up on the Apple TV.

## Custom devices with MQTT

I just recently realized that I can easily use the MQTT integration to create my own custom devices and entities. I known there's a "System monitor" integration, but just to get started, I wrote a bash script that pushes ping, temperature and system usage using `mosquitto_pub`. It was a lot of fun seeing how easy it was to use the integration. The next thing I'm going to write is a "web server request sensor" to monitor which pages are most frequently visited. It won't replace a complete analytics solution, but it will do the trick for my home projects that only has about 10000 requests per week.

## Glucose control

All of the above is mostly just for fun, but the number one integration I use is the LibreLink integration. The official iPhone app from Libre is... awful: You can't pan or zoom the graphs and the notifications is not helpful at all. I started using Sweet Dreams some months ago, which has a much nicer interface and the notifications are very helpful. However, I find myself getting irritated with how many notifications I get. So I've now replaced it with notifications pushed from Home Assistant. This is perfect for two reasons: I'm in complete control of the rules and I can get the values in both mmol/L and mg/dL.

## Summary

I started out using Homebrigde, since I liked the simplicity and the tight integration with HomeKit, but I soon realized that Apple Home simply does not allow you to set up automations the way Home Assistant allow you to. Starting out with Home Assistant was a bit painful though, but that's mainly because the Home Assistant I started out with is no where near as user friendly as the Home Assistant of 2025. It could of course be because I've gotten better at managing it, but it most certainly also have to do with all the things you can do using the web interface, instead of fiddling around with yams config files directly on the server,

I'm very excited to see where Home Assistant is heading, and I hope that the device manufactures makes it easier to integrate new devices, instead of locking down API's.
