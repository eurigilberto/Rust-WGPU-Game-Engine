___
# Quad UI renderer

For blocks, fonts, icons, and atlased textures
All the blocks are going to be rendered with all the faces facing the same direction.\
> **0 is used to disable the feature**, the data is sent to the pixel shader from the vertex shader

## VERTEX BUFFER - INSTANCE

* Rect position and size //This should be stored as a vertex buffer per instance as it is required by all indexes
* Mask index
* Texture Position Index
* Border Radius index
* Color index
	- 0 use the color texture directly 
	//This should not happen if the texture position index is 0 (catch error on the CPU!)
* Texture sample color channel
	- 0 use color directly
	- 1 use R channel as quad mask
	- 2 use G channel as quad mask
	- 3 use B channel as quad mask
	- 4 use A channel as quad mask
	//Maybe extend it later to suport mutlicolored texture samples
* Type ( rect circle )

## Extra storage buffers

This are meant to store data that is not going to be 

* Rect mask - **STORAGE BUFFER**
	
	- ***float4*** (x,y) position center (z,w) width height
	- A single object is only going to use a single one, the mask should be properly computed by the cpu
	- A single rect mask can be reused by multiple elements

* Border Radius - **STORAGE BUFFER**
	- ***float4*** (x,y,z,w) top left, top right, bottom left, bottom right radius (_This is measured in pixels_)

* Texture position - **STORAGE BUFFER**
	
	- ***float3*** (x,y) uv position (z) the texture array selection
	- Considering this is only going to change if the data inside the textures change, then this buffer should be created and not be change until there is a change in the textures array currently in use 
	- A copy of the buffer with more information should be kept in the CPU for the other systems to be able to use it easily

* Color - **STORAGE BUFFER**
	- Type 1
		- ***float4*** color
	- Type 2
		- ***float4*** color 0
		- ***float4*** color 1
		- ***float4*** uv start - end

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

**HOW TO HAVE THE FONT LAYOUT WORK, WITHOUT DIRECTLY KNOWING HOW THE GUI RENDER SYSTEM WORKS?**\
Maybe create an system that is going to be in charge of translating the resulting font layout into usable data by the gui system. This way the font layout can be used without the GUI system.
___
# LINE RENDERER

Optimized to render lines using triangles