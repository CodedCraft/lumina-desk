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

## CAD Model
### Design Philosophy

The CAD model for LuminaDesk embodies a design philosophy centered around accessibility, ease of assembly, and minimal additional hardware. Here are some key features of the CAD model:

    No Supports: Every component of the CAD model is designed to be 3D printable without the need for supports, streamlining the printing process and ensuring easy reproduction by enthusiasts.

    Minimal Hardware: With the exception of a bearing, which is essential for smooth rotary motion, the design minimizes the need for additional hardware. Even the spring mechanism is replaced with a 3D printed flexure, reducing complexity and cost.

    Snap Fits and Compliant Spring: Assembly is made effortless through intuitive snap fits, eliminating the need for glue, screws, or other fasteners. The compliant spring, also 3D printed, provides the necessary tension and flexibility for smooth operation without relying on traditional hardware components.

### Parametric Modeling

The CAD files provided are accessible via Onshape and are designed with parametric modeling techniques. This means that components can be easily customized and adapted to suit individual preferences or specific requirements without the need to redesign from scratch.
Links to CAD Files

Explore the CAD files for LuminaDesk components below:

### Links to CAD files
#### Lamp 
- [View in Onshape as parametric model](https://cad.onshape.com/documents/facbb23b337033b2b3b3d6ee/w/e6ad57cbdafc9e421006821e/e/b91d026dc89f9bd327e5d08e?renderMode=0&uiState=65cf92a12810553b67fab8ff)
#### Rotary Dial
- [View in Onshape as parametric model](https://cad.onshape.com/documents/149174eb9bc19ea455541905/w/f8a6decdda697983f2bb9c4a/e/b882fa6dbc5505b2fd5661fd)
#### Electronics Housing & Lamp Holder
- [View in Onshape as parametric model](https://cad.onshape.com/documents/facbb23b337033b2b3b3d6ee/w/e6ad57cbdafc9e421006821e/e/b91d026dc89f9bd327e5d08e?renderMode=0&uiState=65cf92a12810553b67fab8ff)
