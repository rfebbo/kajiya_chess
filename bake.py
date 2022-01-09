import os
import subprocess


# path to Kajiya bake program
BAKE = '../kajiya/target/release/bake'

meshes_dir = "./assets/meshes/"
baked_dir = './baked/' #internal to Kajia cannot be modified here

# scale models by directory name. (defaults to 1.0 if not in this dictionary)
model_scales = {
    '336_lrm' : 0.01,
    'chess_board' : 1.0,
    'chess_peices' : 0.025,
    'tri' : 0.01
}



def compile_chess():
    scad_file_dir = './assets/scad/'
    scad_files_dir = "./assets/scad/scad_files/"
    stl_outputs_dir = "./assets/scad/stl_outputs/"
    gltf_outputs_dir = "./assets/meshes/chess_peices/"


    subprocess.run(['mkdir', '-p', stl_outputs_dir])
    subprocess.run(['mkdir', '-p', gltf_outputs_dir])

    scad_files = os.listdir(scad_files_dir)
    stl_outputs = os.listdir(stl_outputs_dir)

    for f in scad_files:
        if '.scad' in f:
            model_name = f.split('.')[0]
            if model_name + ".stl" in stl_outputs:
                print(model_name + ".stl found in " + stl_outputs_dir)
            else:
                print("processing " + f + " into stl" ) 
                subprocess.run(["openscad", "-o", stl_outputs_dir + f.split('.')[0] + ".stl",scad_files_dir + f], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


    gltf_outputs = os.listdir(gltf_outputs_dir)
    stl_outputs = os.listdir(stl_outputs_dir)
    for f in stl_outputs:
        model_name = f.split('.')[0]


        if model_name + ".gltf" in gltf_outputs:
            print(model_name + ".gltf found in " + gltf_outputs_dir)
        else:
            print("processing " + f + "into gltf")
            subprocess.run(["blender", "-b", '-P', './2gltf2.py', stl_outputs_dir + f, gltf_outputs_dir + model_name + '.gltf'])
            # subprocess.run(["mv", scad_file_dir + model_name + ".bin", gltf_outputs_dir])
            # subprocess.run(["mv", scad_file_dir + model_name + ".gltf", gltf_outputs_dir])

# compiles gltf files into "internal flat format" for Kajiya
def compile_gltf():
    
    subprocess.run(['mkdir', '-p', baked_dir])
    meshes = os.listdir(meshes_dir)
    baked_folders = os.listdir(baked_dir)

    for folder in meshes:
        scale = 1.0

        if folder in model_scales:
            scale = model_scales[folder]
        

        baked_files = []
        if folder in baked_folders:
            baked_files = os.listdir(baked_dir + folder)
            

        for file in os.listdir(meshes_dir + folder):
            model_name = file.split('.')[0]
            
            if '.gltf' in file:
                # if model_name + '.mesh' in baked_files and os.path.getmtime(baked_dir + folder + '/' + model_name + '.mesh') > os.path.getmtime(meshes_dir + folder + '/' + file):
                #     print(folder + '/' + file + " already compiled into .mesh")
                # else:
                subprocess.run(['mkdir', '-p', './baked/' + folder])
                subprocess.run([BAKE, "--scene", meshes_dir + folder + "/" + file, "--scale", str(scale), "-o", folder + '/' + model_name])



if __name__ == "__main__":


    # compile scad chess peices into stl then into gltf
    compile_chess()

    compile_gltf()

