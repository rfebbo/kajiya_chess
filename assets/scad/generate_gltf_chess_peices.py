import os
import subprocess


if __name__ == "__main__":
    scad_files_dir = "./scad_files/"
    stl_outputs_dir = "./stl_outputs/"
    gltf_outputs_dir = "../meshes/chess_peices/"
    scad_files = os.listdir(scad_files_dir)
    stl_outputs = os.listdir(stl_outputs_dir)

    for f in scad_files:
        if '.scad' in f:
            model_name = f.split('.')[0]
            if model_name + ".stl" in stl_outputs:
                print(model_name + ".stl found in " + stl_outputs_dir)
            else:
                print("processing " + f + "into stl" ) 
                subprocess.run(["openscad", "-o", f"./stl_outputs/{f.split('.')[0]}.stl",scad_files_dir + f], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


    gltf_outputs = os.listdir(gltf_outputs_dir)
    stl_outputs = os.listdir(stl_outputs_dir)
    for f in stl_outputs:
        model_name = f.split('.')[0]


        if model_name + ".gltf" in gltf_outputs:
            print(model_name + ".gltf found in " + gltf_outputs_dir)
        else:
            print("processing " + f + "into gltf")
            subprocess.run(["blender", "-b", '-P', './2gltf2.py', stl_outputs_dir + f])
            subprocess.run(["mv", model_name + ".bin", gltf_outputs_dir])
            subprocess.run(["mv", model_name + ".gltf", gltf_outputs_dir])