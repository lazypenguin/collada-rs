# collada-rs
A rust library for building, parsing and writing collada documents

## Known issues
- Namespaces in `technique` are ignored. This is a current limitation of the xmltree crate
	- E.g. In `<technique xmlns:max="some/schema"><max:someElement>foo</max></technique>`
		the `xmlns:max` attribute is entirely ignored and the namespace `max:` in the 
		`someElement` element is ignored
- `type` attributes are named `typ` due to reserved names in rust 

## Improvements

## Elements to-do
The list of elements that still currently need implementations

### CORE
- [ ] **Animations**
 - [ ] animation
 - [ ] animation_clip
 - [ ] channel
 - [ ] instance_animation
 - [ ] library_animation_clips
 - [ ] library_animations
 - [ ] sampler
- [ ] **Camera**
 - [ ] camera
 - [ ] imager
 - [ ] instance_camera
 - [ ] instance_cameras
 - [ ] library_cameras
 - [ ] optics
 - [ ] orthographic
 - [ ] perspective
- [ ] **Controller**
 - [ ] Controller
 - [ ] instance_controller
 - [ ] joints
 - [ ] library_controllers
 - [ ] morph
 - [ ] skeleton
 - [ ] skin
 - [ ] targets
 - [ ] vertex_weights
- [ ] **Data Flow**
 - [ ] accessor
 - [ ] bool_array
 - [ ] float_array
 - [ ] IDREF_array
 - [ ] int_array
 - [ ] Name_arraay
 - [ ] param (core)
 - [ ] SIDREF_array
 - [ ] source
 - [ ] input (shared)
 - [ ] input (unshared)
- [ ] **Extensibility**
 - [x] extra
 - [x] ~~technique (core)~~
 - [ ] technique_common
- [ ] **Geometry**
 - [ ] control_vertices
 - [ ] geometry 
 - [ ] instance_geometry
 - [ ] library_geometries
 - [ ] lines
 - [ ] linestrips
 - [ ] mesh
 - [ ] polgygons
 - [ ] polylist
 - [ ] spline
 - [ ] triangles
 - [ ] trifans
 - [ ] tristrips 
 - [ ] vertices
- [ ] **Lighting**
 - [ ] ambient (core)
 - [ ] color
 - [ ] directional
 - [ ] instance_light
 - [ ] library_lights
 - [ ] light 
 - [ ] point
 - [ ] spot 
- [ ] **Mathematics**
 - [ ] formula
 - [ ] instance_formula
 - [ ] library_formulas
- [x] **Metadata**
 - [x] asset
 - [x] ~~contributor~~
 - [x] ~~geographic_location~~
- [ ] **Parameters**
 - [ ] newparam
 - [ ] param(reference)
 - [ ] setparam
- [ ] **Scene**
 - [ ] evaluate_scene
 - [ ] instance_node
 - [ ] instance_visual_scene
 - [ ] library_nodes
 - [ ] library_visual_scenes
 - [ ] node
 - [ ] scenes
 - [ ] visual_scene
- [ ] **Transform**
 - [ ] lookat
 - [ ] matrix
 - [ ] rotate
 - [ ] scale
 - [ ] skew
 - [ ] translate
