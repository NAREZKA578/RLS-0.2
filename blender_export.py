# Blender Python Script for RTGC Model Export
# Run this in Blender: File > Export > Run Script, or paste in Scripting tab
# Save as .blend with your model, then run: blender --background model.blend --python export_model.py

import bpy
import os
import math

class ModelExporter:
    def __init__(self, output_dir="."):
        self.output_dir = output_dir
        self.vertices = []
        self.normals = []
        self.colors = []
        self.indices = []
        self.index_offset = 0

    def clear(self):
        self.vertices = []
        self.normals = []
        self.colors = []
        self.indices = []
        self.index_offset = 0

    def process_object(self, obj):
        if obj.type != 'MESH':
            return

        mesh = obj.data
        if not hasattr(mesh, 'vertices'):
            return

        matrix = obj.matrix_world
        color = self.get_object_color(obj)

        mesh.calc_loop_triangles()
        mesh.calc_normals_split()

        if mesh.loop_triangles:
            loop_indices = [lt.vertices for lt in mesh.loop_triangles]
        else:
            mesh.calc_triangles()
            loop_indices = mesh.loop_triangles

        for vert in mesh.vertices:
            pos = matrix @ vert.co
            self.vertices.append((pos.x, pos.y, pos.z))
            self.normals.append((0.0, 1.0, 0.0))

        color_tuple = self.hex_to_rgb(color) if isinstance(color, str) else color
        self.colors.append(color_tuple)

        for tri in loop_indices:
            for vert_idx in tri:
                self.indices.append(self.index_offset + vert_idx)

        self.index_offset += len(mesh.vertices)

    def get_object_color(self, obj):
        color = (0.75, 0.72, 0.68)
        if hasattr(obj, 'color'):
            c = obj.color
            if isinstance(c, tuple) and len(c) >= 3:
                color = (c[0], c[1], c[2])
        return color

    def hex_to_rgb(self, hex_color):
        hex_color = hex_color.lstrip('#')
        lv = len(hex_color)
        return tuple(int(hex_color[i:i+lv//3], 16)/255.0 for i in range(0, lv, lv//3))

    def export_to_simple_mesh(self, filename):
        filepath = os.path.join(self.output_dir, filename)
        with open(filepath, 'w') as f:
            f.write(f"# Vertices: {len(self.vertices)}\n")
            f.write(f"# Indices: {len(self.indices)}\n\n")

            f.write("VERTICES\n")
            for v in self.vertices:
                f.write(f"v {v[0]:.4f} {v[1]:.4f} {v[2]:.4f}\n")

            f.write("\nNORMALS\n")
            for n in self.normals:
                f.write(f"n {n[0]:.4f} {n[1]:.4f} {n[2]:.4f}\n")

            f.write("\nCOLORS\n")
            for c in self.colors:
                f.write(f"c {c[0]:.4f} {c[1]:.4f} {c[2]:.4f}\n")

            f.write("\nINDICES\n")
            for i in range(0, len(self.indices), 3):
                if i + 2 < len(self.indices):
                    f.write(f"f {self.indices[i]} {self.indices[i+1]} {self.indices[i+2]}\n")

        print(f"Exported to {filepath}")
        return filepath

def export_selected():
    exporter = ModelExporter()

    for obj in bpy.context.selected_objects:
        exporter.process_object(obj)

    if exporter.vertices:
        exporter.export_to_simple_mesh("exported_model.txt")

def export_all():
    exporter = ModelExporter()
    for obj in bpy.data.objects:
        exporter.process_object(obj)

    if exporter.vertices:
        exporter.export_to_simple_mesh("exported_model.txt")

def export_uaz():
    exporter = ModelExporter()

    body_color = (0.75, 0.72, 0.68)
    wheel_color = (0.15, 0.15, 0.15)
    trim_color = (0.4, 0.4, 0.4)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0, 1.5))
    body = bpy.context.active_object
    body.scale = (2, 4, 0.8)
    bpy.ops.object.transform_apply(scale=True)
    exporter.process_object(body)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0, 2.2))
    cabin = bpy.context.active_object
    cabin.scale = (1.8, 2.5, 0.7)
    bpy.ops.object.transform_apply(scale=True)
    exporter.process_object(cabin)

    wheel_positions = [(1.8, -2.5, 0), (1.8, 2.5, 0), (-1.8, -2.5, 0), (-1.8, 2.5, 0)]
    for pos in wheel_positions:
        bpy.ops.mesh.primitive_cylinder_add(radius=0.5, depth=0.4, location=pos)
        wheel = bpy.context.active_object
        wheel.rotation_euler = (math.pi/2, 0, 0)
        bpy.ops.object.transform_apply(rotation=True)
        exporter.process_object(wheel)

    exporter.export_to_simple_mesh("uaz_simple.txt")

def export_terrain():
    exporter = ModelExporter()

    size = 50
    step = 2
    base_y = 0

    for z in range(-size, size+1, step):
        for x in range(-size, size+1, step):
            from random import random
            height = random() * 0.5

            bpy.ops.mesh.primitive_cube_add(size=1, location=(x, z, base_y + height/2))
            cube = bpy.context.active_object
            cube.scale = (step/2, step/2, height + 0.1)
            bpy.ops.object.transform_apply(scale=True)

            col = (0.25 + random()*0.2, 0.42 + random()*0.1, 0.18 + random()*0.1)
            if hasattr(cube, 'color'):
                cube.color = (*col, 1.0)

            exporter.process_object(cube)

    exporter.export_to_simple_mesh("terrain_simple.txt")

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        if sys.argv[-1] == "uaz":
            export_uaz()
        elif sys.argv[-1] == "terrain":
            export_terrain()
        else:
            export_selected()
    else:
        export_selected()