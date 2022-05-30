# Entity Component system
The component system for the engine is going to be a unity style one (to a certain extent).

The most important processes that need to go throw multiple entities are: (in the orther they are going to be executed)
1. Winit events + Gamepad Events (events from winit, are going to be in a queue)
2. Update components
3. Render (All command buffers are going to be submitted here)

## How to structure the GUI system
The GUI structure - implemented as if it was an external crate, to a certain extent

From the specified processes above some part of the GUI system needs to:
1. Handle the input events
	- GUI render textures need to be resized if the window is resized.
2. Update the window elements
3. Submit the render command - Calls the "render" event on the window elements, which is going to update the data buffers that are going to be sent in the UI draw call. I am still unsure about how the extra elements should be handled. The extra elements need the generated mask texture to render to the screen properly, and they are going to use the same color texture to render themselfs, but where is their data going to be stored? and how are they going to be interated over?. The order in which those elements need to be rendered, is going to be computed by the "Update" function in the "Window" elements.

A needed change from the previous system is that the "window" elements should have an update function that does not assume anything about where they are being rendered, which would make it easier for them to be used as part of other windowing schemes, like tabs.

The GUI windows entities cannot be updated directly from the other entities, if some data needs to be sent to an specific window then a public data entity has to be updated instead, and the UI should change its layout accordingly.