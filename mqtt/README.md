# mrnr proto mqtt

Sample MQTT client written in Rust (by Alex Helfet) and packed as HASS add-on by me.
- Build on demand on deployment.
- Discover MQTT connection parameters on startup.
- Debian based Home Assistant Base Image (glibc client req).
- Publish static payload "42" on "homeassistant/sensor/sensorBedroomT/state" topic.

Thanks to:
- [MQTT Client](https://github.com/fluffysquirrels/mqtt-async-client-rs) 
- [Add-on Docs](https://developers.home-assistant.io/docs/add-ons)
- [Add-on Base](https://github.com/hassio-addons/addon-base)
- [Base Images](https://github.com/home-assistant/docker-base)
