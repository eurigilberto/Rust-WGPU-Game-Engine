___
# Quad UI renderer

For blocks, fonts, icons, and atlased textures
All the blocks are going to be rendered with all the faces facing the same direction.\
> **0 is used to disable the feature**, the data is sent to the pixel shader from the vertex shader

## VERTEX BUFFER - INSTANCE

* Rect position and size - **vec4\<f32\>**
* Mask index - **u32**
* Texture Position Index - **u32**
* Border Radius index - **u32**
* Color index - **u32**
	- 0 use the color texture directly 
		>This should not happen if the texture position index is 0 (catch error on the CPU!)
* Texture sample color channel - **u32**
	- 0 use color directly
	- 1 use R channel as quad mask
	- 2 use G channel as quad mask
	- 3 use B channel as quad mask
	- 4 use A channel as quad mask
	//Maybe extend it later to suport mutlicolored texture samples
* Type - **u32**
	- (0) rect - rounded rect sdf
	- (1) font - font (single channel) sdf
	- (2) circle - circle sdf
	- (3) texture render - (behaviour depends on the texture sample color channel value)
		- Single channel : color = (color_index.rgb, sample.r)
		- Fully colored : color = (color_index.rgba)

## Extra storage buffers

This are meant to store data that is not going to be 

* Rect mask - **STORAGE BUFFER**
	
	- **vec4\<f32\>** (x,y) position center (z,w) width height
	- A single object is only going to use a single one, the mask should be properly computed by the cpu
	- A single rect mask can be reused by multiple elements

* Border Radius - **STORAGE BUFFER**
	- **vec4\<f32\>** (x,y,z,w) top left, top right, bottom left, bottom right radius (_This is measured in pixels_)

* Texture position - **STORAGE BUFFER**
	
	- **vec4\<u32\>** 
		- **x , y** top left corner position of the `texture slice`.
		- **z** a packed u32 that holds the `size` of the `texture slice`.
		- **w** the texture array selection.
	- Considering this is only going to change if the data inside the textures change, then this buffer should be created and not be change until there is a change in the textures array currently in use 
	- A copy of the buffer with more information should be kept in the CPU for the other systems to be able to use it easily

* Color - **STORAGE BUFFER**
	- Type 1
		- **vec4\<f32\>** color
	- Type 2
		- **vec4\<f32\>** color 0
		- **vec4\<f32\>** color 1
		- **vec4\<f32\>** uv start - end

## 2D Texture Array

This is a collection of textures that is going to be used by the GUI system to render fonts and other textures that do not change from frame to frame, like fonts, icons and other UI related elements.

Because it is a texture array, the same size is going to be used for all textures in the array. This size is going to be used on the font generation and it also needs to be used for any other required texture. In the future a **texture packer** could be created on the engine to allow for textures that are smaller to be packed together more efficiently.
___
## Desired usage
___
### Font Atlas
Using the **Font Atlas** genearte as many font textures as needed. Keep in mind that the current version always tries to load all the characters defined in the TTF file, a future version should enable the usage of a character set. The font atlas is going to require:
- ***texture size***
- ***character render size***
- ***character sdf padding***
- ***ttf file path*** 

> Depending on the _texture size and character render size + padding_ requested the operation might fail as there is not enought space.

This atlas creation is going to result in a array of R16 float textures that need to be packed into the RGBA channels of the textures in the array.

### Icons and UI elements
If there are others icons that are not aprt of a font, then they are going to be placed using the same layout system that is going to be created byt he UI.

> **Position Buffer**\
A `CPUGPUBuffer` needs to be created to hold the information about the sections of the texture
___
# LINE RENDERER

Optimized to render lines using triangles