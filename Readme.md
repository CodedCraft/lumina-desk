# LuminaDesk
LuminaDesk, an innovative monitor lamp, offers even illumination for your workspace. 3D printable, easy to assemble and with minimal additional hardware. All at a fraction of the cost of commercial alternatives. Despite its affordability, LuminaDesk excels in features, boasting a superior reflector to eliminate spillage on your monitor, ensuring a seamlessly lit desk. 

## Features
### Wireless Smart Control
LuminaDesk comes equipped with a wireless control dial, providing a seamless and intuitive user experience. The control dial incorporates a position sensor (AS5600) for rotary encoding and a tactile button, ensuring precise and effortless control over your lighting preferences.

### Clever CAD Model
The CAD for LuminaDesk was modeled with ease of assembly and minimal part count as the main requirment. Forget the hassle of glue, screws, or other fasteners – this design relies on intuitive snap fits, with a compliant spring eliminating the need for additional hardware. Plus, every part is designed to be printed without requiring supports. This deliberate design philosophy breaks down barriers, ensuring LuminaDesk is not only accessible but also an easy endeavor for enthusiasts to replicate and enjoy.

### Glare-Free Illumination
The carefully engineered reflector of LuminaDesk is designed to eliminate reflections on your monitor, providing a glare-free environment for maximum productivity. The lamp is engineered to illuminate your desk without causing any discomfort or eye strain, ensuring an optimal workspace ambiance.

### Dynamic Lighting Control
LuminaDesk offers a versatile lighting experience with dimming functionality and adjustable color temperature. Tailor the lighting to your preferences and the task at hand, creating the perfect atmosphere for work or relaxation.

### Intelligent Mode Switching (Multi-tap gestures)
Switch between dimming and color temperature adjustment modes effortlessly with a double press of the control dial. A long press on the control dial allows you to save your preferred brightness and color-temperature settings. Retrieving this preset is as easy as executing a quick triple tap. The choice of multi-tap gestures is intentional, designed for simplicity and to minimize the need for additional tactile buttons, keeping LuminaDesk's interface elegant and user-friendly.

## Software implementation
### Powered by ESP32-C3
At the heart of LuminaDesk is the powerful ESP32 C3 microcontroller, delivering robust performance and efficient energy management. This enables smooth communication over ESP-NOW, ensuring a reliable connection between the lamp and its control interface.

### Rust and ESP-IDF-HAL Integration
LuminaDesk's firmware is written in Rust, showcasing a commitment to modern programming practices. The integration of the ESP-IDF-HAL enhances interoperability with C, providing a robust foundation for firmware development.

### Energy-Efficient Design
The LuminaDesk control dial is programmed to go into deep sleep whenever possible, conserving energy and promoting sustainability. This eco-friendly design ensures long-lasting performance while minimizing environmental impact.

## Commercial Alternatives

For those leaning towards ready-made solutions, LuminaDesk drew inspiration from several commercial alternatives:

#### BenQ ScreenBar/ ScreenBar Halo:
- Best-in-class illumination with a superior reflector, ensuring optimal brightness.
- Premium quality comes at a higher cost.

#### Xiaomi Mi Light Bar:
- Feature-rich and budget-friendly, yet with a less-than-optimal reflector.
- A balance of functionality and affordability for those seeking convenience.

#### Quntis Light Bar:
- The most economical option but prone to light spillage on the monitor.
- A cost-effective choice for those prioritizing budget over advanced features.

These alternatives showcase the diversity in the market. LuminaDesk, inspired by these choices, presents an open-source, customizable solution for enthusiasts who relish a hands-on approach to perfecting their workspace lighting.
