# 
# The MIT License (MIT)
#
# Copyright (c) since 2017 UX3D GmbH
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
# 

#
# Imports
#

import bpy
import os
import sys

#
# Globals
#

#
# Functions
#


print("Converting: '" + sys.argv[4] + "'")
current_argument = sys.argv[4]
current_extension = '.' + sys.argv[4].split('.')[2]
print(current_extension)
bpy.ops.wm.read_factory_settings(use_empty=True)

#

if current_extension == ".abc":
    bpy.ops.wm.alembic_import(filepath=current_argument)    

elif current_extension == ".blend":
    bpy.ops.wm.open_mainfile(filepath=current_argument)

elif current_extension == ".dae":
    bpy.ops.wm.collada_import(filepath=current_argument)    

elif current_extension == ".fbx":
    bpy.ops.import_scene.fbx(filepath=current_argument)    

elif current_extension == ".obj":
    bpy.ops.import_scene.obj(filepath=current_argument)    

elif current_extension == ".ply":
    bpy.ops.import_mesh.ply(filepath=current_argument)    

elif current_extension == ".stl":
    print("AS STL")
    bpy.ops.import_mesh.stl(filepath=current_argument)

elif current_extension == ".usd" or current_extension == ".usda" or current_extension == ".usdc":
    bpy.ops.wm.usd_import(filepath=current_argument)

elif current_extension == ".wrl" or current_extension == ".x3d":
    bpy.ops.import_scene.x3d(filepath=current_argument)

#

# export_file = current_directory + "/" + current_basename + ".gltf"
print("Writing: '" + sys.argv[5] + "'")
bpy.ops.export_scene.gltf(filepath=sys.argv[5], export_format='GLTF_SEPARATE')
