import bpy
import os

def load_and_export():
    blend_path = "C:\\Users\\mrkri\\Documents\\RTGC-1.0\\assets\\models\\uaz_model.blend"
    output_path = "C:\\Users\\mrkri\\Documents\\RTGC-1.0\\assets\\models\\uaz_model.obj"

    bpy.ops.wm.open_mainfile(filepath=blend_path)

    obj_file = open(output_path, 'w')
    obj_file.write("# RTGC UAZ Model\n")
    obj_file.write("# Exported from Blender\n\n")

    vertex_offset = 1
    total_verts = 0
    total_faces = 0

    for obj in bpy.data.objects:
        if obj.type != 'MESH':
            continue

        mesh = obj.data
        matrix = obj.matrix_world

        obj_file.write(f"o {obj.name}\n")

        mesh.calc_loop_triangles()

        temp_verts = []
        for vert in mesh.vertices:
            world_pos = matrix @ vert.co
            temp_verts.append(world_pos)

        for tri in mesh.loop_triangles:
            for loop_idx in tri.loops:
                loop = mesh.loops[loop_idx]
                vert_idx = loop.vertex_index
                world_pos = temp_verts[vert_idx]
                normal = loop.normal
                world_normal = matrix.to_3x3() @ normal
                if world_normal.length > 0.0001:
                    world_normal = world_normal.normalized()

                obj_file.write(f"v {world_pos.x:.6f} {world_pos.y:.6f} {world_pos.z:.6f}\n")
                obj_file.write(f"vn {world_normal.x:.6f} {world_normal.y:.6f} {world_normal.z:.6f}\n")
                total_verts += 1

        for tri in mesh.loop_triangles:
            v1 = vertex_offset
            v2 = vertex_offset + 1
            v3 = vertex_offset + 2
            obj_file.write(f"f {v1}//{v1} {v2}//{v2} {v3}//{v3}\n")
            vertex_offset += 3
            total_faces += 1

    obj_file.close()

    file_size = os.path.getsize(output_path)
    print(f"Exported: {total_verts} vertices, {total_faces} triangles")
    print(f"File: {output_path} ({file_size} bytes)")
    return output_path

if __name__ == "__main__":
    load_and_export()