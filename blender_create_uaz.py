import bpy
import math

def create_uaz():
    """Создаёт модель UAZ 469 в Blender"""
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()

    body_color = (0.75, 0.72, 0.68)
    wheel_color = (0.12, 0.12, 0.12)
    trim_color = (0.5, 0.5, 0.5)

    bpy.ops.mesh.primitive_cube_add(size=1, location=(0, 0, 1.0))
    body = bpy.context.active_object
    body.name = "UAZ_Body"
    body.scale = (2.0, 4.5, 0.6)
    bpy.ops.object.transform_apply(scale=True)
    if hasattr(body.data, 'materials'):
        mat = bpy.data.materials.new(name="BodyMaterial")
        mat.diffuse_color = (*body_color, 1.0)
        body.data.materials.append(mat)

    bpy.ops.mesh.primitive_cube_add(size=1, location=(0, 0.5, 1.6))
    cabin = bpy.context.active_object
    cabin.name = "UAZ_Cabin"
    cabin.scale = (1.9, 2.5, 0.6)
    bpy.ops.object.transform_apply(scale=True)
    if hasattr(cabin.data, 'materials'):
        mat = bpy.data.materials.new(name="CabinMaterial")
        mat.diffuse_color = (*body_color, 1.0)
        cabin.data.materials.append(mat)

    bpy.ops.mesh.primitive_cube_add(size=1, location=(0, -1.5, 1.0))
    trunk = bpy.context.active_object
    trunk.name = "UAZ_Trunk"
    trunk.scale = (1.9, 1.5, 0.5)
    bpy.ops.object.transform_apply(scale=True)

    wheel_positions = [
        (2.0, -2.5, 0.4),
        (2.0, 2.5, 0.4),
        (-2.0, -2.5, 0.4),
        (-2.0, 2.5, 0.4),
    ]
    for i, pos in enumerate(wheel_positions):
        bpy.ops.mesh.primitive_cylinder_add(radius=0.4, depth=0.3, location=pos)
        wheel = bpy.context.active_object
        wheel.name = f"UAZ_Wheel_{i}"
        wheel.rotation_euler = (math.pi/2, 0, 0)
        bpy.ops.object.transform_apply(rotation=True)
        if hasattr(wheel.data, 'materials'):
            mat = bpy.data.materials.new(name=f"WheelMaterial_{i}")
            mat.diffuse_color = (*wheel_color, 1.0)
            wheel.data.materials.append(mat)

    bpy.ops.mesh.primitive_cylinder_add(radius=0.15, depth=4.0, location=(0, 0, 1.8))
    bumper_f = bpy.context.active_object
    bumper_f.name = "UAZ_Bumper_Front"
    bumper_f.rotation_euler = (0, math.pi/2, 0)
    bpy.ops.object.transform_apply(rotation=True)

    bpy.ops.mesh.primitive_cylinder_add(radius=0.15, depth=4.0, location=(0, 0, 0.5))
    bumper_b = bpy.context.active_object
    bumper_b.name = "UAZ_Bumper_Back"
    bumper_b.rotation_euler = (0, math.pi/2, 0)
    bpy.ops.object.transform_apply(rotation=True)

    bpy.ops.object.select_all(action='SELECT')

    print("UAZ model created! Save as .blend to keep editing.")
    return bpy.context.scene.objects

def export_for_game(filepath="//..\\assets\\models\\uaz_mesh.txt"):
    """Экспортирует выбранные объекты в простой формат для игры"""
    objects = bpy.context.selected_objects
    if not objects:
        objects = bpy.data.objects

    vertices = []
    normals = []
    colors = []
    indices = []
    index_offset = 0

    for obj in objects:
        if obj.type != 'MESH':
            continue

        matrix = obj.matrix_world
        mesh = obj.data

        color = (0.75, 0.72, 0.68)
        if hasattr(obj, 'color') and len(obj.color) >= 3:
            color = (obj.color[0], obj.color[1], obj.color[2])

        mesh.calc_loop_triangles()
        mesh.calc_normals_split()

        for vert in mesh.vertices:
            pos = matrix @ vert.co
            normals_split = [matrix.to_3x3() @ mesh.loops[i].normal for i in range(len(mesh.loops))]

        for i, vert in enumerate(mesh.vertices):
            pos = matrix @ vert.co
            vertices.append((pos.x, pos.y, pos.z))
            if mesh.loop_triangles:
                n = mesh.loop_triangles[0].split_normals[i] if i < len(mesh.loop_triangles[0].split_normals) else (0,1,0)
                normals.append(n)
            else:
                normals.append((0,1,0))
            colors.append(color)

        for tri in mesh.loop_triangles:
            indices.append((index_offset + tri.vertices[0],
                          index_offset + tri.vertices[1],
                          index_offset + tri.vertices[2]))

        index_offset += len(mesh.vertices)

    with open(filepath, 'w') as f:
        f.write(f"# RTGC Mesh Export\n")
        f.write(f"# Vertices: {len(vertices)}\n")
        f.write(f"# Triangles: {len(indices)}\n\n")

        f.write("VERTEX_BUFFER\n")
        for i, v in enumerate(vertices):
            n = normals[i] if i < len(normals) else (0,1,0)
            c = colors[i] if i < len(colors) else (0.75,0.72,0.68)
            f.write(f"{v[0]:.4f} {v[1]:.4f} {v[2]:.4f}  {n[0]:.4f} {n[1]:.4f} {n[2]:.4f}  {c[0]:.4f} {c[1]:.4f} {c[2]:.4f}\n")

        f.write("\nINDEX_BUFFER\n")
        for tri in indices:
            f.write(f"{tri[0]} {tri[1]} {tri[2]}\n")

    print(f"Exported to {bpy.path.abspath(filepath)}")
    return filepath

if __name__ == "__main__":
    create_uaz()
    export_for_game()