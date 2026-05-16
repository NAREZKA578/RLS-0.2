import bpy
import math
import os

def clear_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)

def create_material(name, color, metallic=0.0, roughness=0.5):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    nodes = mat.node_tree.nodes
    nodes.clear()
    bsdf = nodes.new("ShaderNodeBsdfPrincipled")
    bsdf.location = (0, 0)
    bsdf.inputs['Base Color'].default_value = (*color, 1.0)
    bsdf.inputs['Metallic'].default_value = metallic
    bsdf.inputs['Roughness'].default_value = roughness
    output = nodes.new("ShaderNodeOutputMaterial")
    output.location = (200, 0)
    mat.node_tree.links.new(bsdf.outputs['BSDF'], output.inputs['Surface'])
    return mat

def create_uaz():
    clear_scene()

    body_mat = create_material("BodyMat", (0.72, 0.68, 0.60))
    cabin_mat = create_material("CabinMat", (0.65, 0.60, 0.55))
    wheel_mat = create_material("WheelMat", (0.1, 0.1, 0.1), roughness=0.8)
    glass_mat = create_material("GlassMat", (0.3, 0.4, 0.5), metallic=0.1)
    trim_mat = create_material("TrimMat", (0.4, 0.4, 0.4))
    bumper_mat = create_material("BumperMat", (0.3, 0.3, 0.3))

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0, 0))
    body = bpy.context.active_object
    body.name = "Body"
    body.scale = (2.2, 4.5, 0.5)
    bpy.ops.object.transform_apply(scale=True)
    body.data.materials.append(body_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0.8, 0.7))
    cabin = bpy.context.active_object
    cabin.name = "Cabin"
    cabin.scale = (2.0, 2.8, 0.8)
    bpy.ops.object.transform_apply(scale=True)
    cabin.data.materials.append(cabin_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, -1.8, 0.4))
    trunk = bpy.context.active_object
    trunk.name = "Trunk"
    trunk.scale = (2.0, 1.5, 0.4)
    bpy.ops.object.transform_apply(scale=True)
    trunk.data.materials.append(body_mat)

    wheel_positions = [
        (2.3, -2.5, 0),
        (2.3, 2.5, 0),
        (-2.3, -2.5, 0),
        (-2.3, 2.5, 0),
    ]
    for i, pos in enumerate(wheel_positions):
        bpy.ops.mesh.primitive_cylinder_add(radius=0.5, depth=0.4, location=pos)
        wheel = bpy.context.active_object
        wheel.name = f"Wheel_{i}"
        wheel.rotation_euler = (math.pi/2, 0, 0)
        bpy.ops.object.transform_apply(rotation=True)
        wheel.data.materials.append(wheel_mat)

    bpy.ops.mesh.primitive_cylinder_add(radius=0.12, depth=0.3, location=(2.2, -2.5, 0))
    hub = bpy.context.active_object
    hub.name = "Hub_0"
    hub.scale = (1, 1, 0.5)
    hub.rotation_euler = (math.pi/2, 0, 0)
    bpy.ops.object.transform_apply(scale=True)
    bpy.ops.object.transform_apply(rotation=True)
    hub.data.materials.append(trim_mat)

    bpy.ops.mesh.primitive_cylinder_add(radius=0.12, depth=0.3, location=(2.2, 2.5, 0))
    hub = bpy.context.active_object
    hub.name = "Hub_1"
    hub.scale = (1, 1, 0.5)
    hub.rotation_euler = (math.pi/2, 0, 0)
    bpy.ops.object.transform_apply(scale=True)
    bpy.ops.object.transform_apply(rotation=True)
    hub.data.materials.append(trim_mat)

    bpy.ops.mesh.primitive_cylinder_add(radius=0.12, depth=0.3, location=(-2.2, -2.5, 0))
    hub = bpy.context.active_object
    hub.name = "Hub_2"
    hub.scale = (1, 1, 0.5)
    hub.rotation_euler = (math.pi/2, 0, 0)
    bpy.ops.object.transform_apply(scale=True)
    bpy.ops.object.transform_apply(rotation=True)
    hub.data.materials.append(trim_mat)

    bpy.ops.mesh.primitive_cylinder_add(radius=0.12, depth=0.3, location=(-2.2, 2.5, 0))
    hub = bpy.context.active_object
    hub.name = "Hub_3"
    hub.scale = (1, 1, 0.5)
    hub.rotation_euler = (math.pi/2, 0, 0)
    bpy.ops.object.transform_apply(scale=True)
    bpy.ops.object.transform_apply(rotation=True)
    hub.data.materials.append(trim_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0, 1.4))
    windshield = bpy.context.active_object
    windshield.name = "Windshield"
    windshield.scale = (1.8, 0.1, 0.6)
    bpy.ops.object.transform_apply(scale=True)
    windshield.data.materials.append(glass_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, -0.6, 1.15))
    hood = bpy.context.active_object
    hood.name = "Hood"
    hood.scale = (1.9, 0.8, 0.15)
    bpy.ops.object.transform_apply(scale=True)
    hood.data.materials.append(body_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(2.4, 0, 0.25))
    fender_r = bpy.context.active_object
    fender_r.name = "Fender_R"
    fender_r.scale = (0.15, 4.5, 0.3)
    bpy.ops.object.transform_apply(scale=True)
    fender_r.data.materials.append(body_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(-2.4, 0, 0.25))
    fender_l = bpy.context.active_object
    fender_l.name = "Fender_L"
    fender_l.scale = (0.15, 4.5, 0.3)
    bpy.ops.object.transform_apply(scale=True)
    fender_l.data.materials.append(body_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 4.7, 0.3))
    bumper_f = bpy.context.active_object
    bumper_f.name = "Bumper_Front"
    bumper_f.scale = (2.5, 0.2, 0.3)
    bpy.ops.object.transform_apply(scale=True)
    bumper_f.data.materials.append(bumper_mat)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, -4.7, 0.3))
    bumper_b = bpy.context.active_object
    bumper_b.name = "Bumper_Back"
    bumper_b.scale = (2.5, 0.2, 0.3)
    bpy.ops.object.transform_apply(scale=True)
    bumper_b.data.materials.append(bumper_mat)

    bpy.ops.object.select_all(action='SELECT')

    blend_path = "C:\\Users\\mrkri\\Documents\\RTGC-1.0\\assets\\models\\uaz_model.blend"
    os.makedirs(os.path.dirname(blend_path), exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=blend_path)

    print(f"UAZ model saved to: {blend_path}")

    return blend_path

if __name__ == "__main__":
    create_uaz()