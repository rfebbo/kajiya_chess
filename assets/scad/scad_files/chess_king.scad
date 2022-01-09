segments = 64;

scale(0.185)
rotate([0, 0, 0])
translate([0, 0, 0]) {

translate([0, 0, 277])
union () {
    rotate_extrude(convexity = 10, $fn = segments) {
	import_dxf(file = "./dxf_profiles/king_profile.dxf");
    }
    translate([-25, 5, 40])
    rotate([90, 0, 0])
    linear_extrude(height = 10) {
	import_dxf(file = "./dxf_profiles/cross_profile.dxf");
    }
}

}
