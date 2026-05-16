import bpy

def export_selected():
    output_path = "C:\\Users\\mrkri\\Documents\\RTGC-1.0\\assets\\models\\uaz_model.obj"

    bpy.ops.export_scene.obj(
        filepath=output_path,
        use_selection=True,
        use_mesh_modifiers=True,
        use_normals=True,
        use_uvs=False,
        use_materials=False,
        axis_forward='Y',
        axis_up='Z',
    )

    print(f"Exported to: {output_path}")

def export_all():
    output_path = "C:\\Users\\mrkri\\Documents\\RTGC-1.0\\assets\\models\\uaz_model.obj"

    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.export_scene.obj(
        filepath=output_path,
        use_selection=False,
        use_mesh_modifiers=True,
        use_normals=True,
        use_uvs=False,
        use_materials=False,
        axis_forward='Y',
        axis_up='Z',
    )

    print(f"Exported to: {output_path}")

if __name__ == "__main__":
    export_all()