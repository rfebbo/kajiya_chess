segments = 64;

scale(0.185)
rotate([0, 0, 0])
translate([0, 0, 0]) {

translate([0, 0, 216])
union () {
    rotate_extrude(convexity = 10, $fn = segments) {
	import_dxf(file = "./dxf_profiles/queen_profile.dxf");
    }
    // Add the crown
    translate([0, 0, 28])
    scale(7.0)
    import(file = "./stl_imports/queen_crown2.stl");
}

}
