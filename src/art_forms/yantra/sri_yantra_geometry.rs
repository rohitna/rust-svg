//! A library to construct Shri Yantra like objects see [Wiki](https://en.wikipedia.org/wiki/Sri_Yantra).

// use geo::prelude::*;
use crate::utils::intersection; // {dist, intersection, polar_point};
use geo::prelude::*;
use geo::{CoordFloat, GeoFloat, Point};
use numeric_literals::replace_numeric_literals;
use std::collections::HashMap;

#[derive(Clone)]
/// # `ShriYantra` struct details.
/// A Sri Yantra consists of four up triangles and 5 down triangle setup inscribed in a circle (plus
/// additional stuff like bhūpura which we ignore here for simplicity, see [Wiki](https://en.wikipedia.org/wiki/Sri_Yantra)).
/// Triangles intersects so as to form 5 paths.
///
/// - The paths look like lotuses with petals 14 + 10 + 10 + 8 + 1 = 43.
/// - Shri yantra has some triple intersection and concurrency requirements,
///     after which it still has four degrees of freedom (see [Chiodo](https://doi.org/10.5802/crmath.163))
///     which one can think of as five parameters satisfying one equation.
/// - The parameters here are named as in this paper by [Fonseca](http://dx.doi.org/10.1016/0048-721x(86)90004-7).
pub struct ShriYantra<T: CoordFloat = f64> {
    /// Radius of the inscribing circle.
    pub radius: T,
    /// Center of the inscribing circle.
    pub center: Point<T>,
    /// Length of `XA` as in [Fonseca](http://dx.doi.org/10.1016/0048-721x(86)90004-7).
    pub param_a: T,
    /// Length of `XC` as in [Fonseca](http://dx.doi.org/10.1016/0048-721x(86)90004-7).
    pub param_c: T,
    /// Length of `XF` as in [Fonseca](http://dx.doi.org/10.1016/0048-721x(86)90004-7).
    pub param_f: T,
    /// Length of `XG` as in [Fonseca](http://dx.doi.org/10.1016/0048-721x(86)90004-7).
    pub param_g: T,
    /// Length of `XI` as in [Fonseca](http://dx.doi.org/10.1016/0048-721x(86)90004-7).
    pub param_i: T,
    /// Map that takes a coordinate name to its point value.
    coords: HashMap<CoordName, Point<T>>,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
/// A dictionary of points in the Sri Yantra
enum CoordName {
    // U denote an up triangle, D a down triangle,
    // 1, 2, .. denote their sizes starting from the largest point
    // T is tip, M is the mid point of the base, L and R are left and right vertices
    // If a point can have two names, one one them is picked randomly
    UT1,
    UL1,
    UR1,
    UM1,
    DT1,
    DL1,
    DR1,
    DM1,
    UT2,
    UL2,
    UR2,
    DT2,
    DR2,
    DL2,
    UT3,
    UL3,
    UR3,
    UM3,
    DT3,
    DL3,
    DR3,
    UL4,
    UR4,
    DL4,
    DR4,
    DR5,
    DL5,
    // Triple intersections with "apparant" C4 symmetry
    NWG1, // On first path with lower |y|
    NEG1,
    SWG1,
    SEG1,
    NWG2, // On first path with higher |y|
    NEG2,
    SWG2,
    SEG2,
    NWG3, // On second path
    NEG3,
    SWG3,
    SEG3,
    NWG4, // On the third path
    NEG4,
    SWG4,
    SEG4,
    // The remaining triple point on the fourth path
    EG,
    WG,
    // Remaining double intersections on first path
    EH1,
    WH1,
    NWH,
    NEH,
    SWH,
    SEH,
    // Remaining double intersections on second path starting from the top
    EI1,
    WI1,
    EI2,
    WI2,
    EI3,
    WI3,
    // Remaining double intersections on third path
    EK,
    WK,
    NWF, // the one with "apparant" c4 symmetry
    NEF,
    SWF,
    SEF,
    // Double intersections on the fourth path starting from top
    EJ1,
    WJ1,
    EJ2,
    WJ2,
    EJ3,
    WJ3,
    // Bindu
    BINDU,
}

#[replace_numeric_literals(T::from(literal).unwrap())]
impl<T: CoordFloat + GeoFloat> ShriYantra<T> {


    /// Create a new `ShriYantra`.
    pub fn new(
        radius: T,
        center: Point<T>,
        param_a: Option<T>,
        param_c: Option<T>,
        param_f: Option<T>,
        param_g: Option<T>,
        param_i: Option<T>,
    ) -> Self {
        let diameter = radius * 2.0;
        // Some optimal values of (a,f):
        // (a, f) = (5.0, 26.5), (6, 26.1), (6.2, 26.0)
        ShriYantra {
            radius: radius,
            center: center,
            param_a: param_a.unwrap_or(diameter * 5.0 / 48.0),
            param_c: param_c.unwrap_or(diameter * 17.0 / 48.0),
            param_f: param_f.unwrap_or(diameter * 26.5 / 48.0),
            param_g: param_g.unwrap_or(diameter * 30.0 / 48.0),
            param_i: param_i.unwrap_or(diameter * 42.0 / 48.0),
            coords: HashMap::<CoordName, Point<T>>::new(),
        }
    }

    /// Get all contructed points a `SriYantra`.
    pub fn get_all_points(&mut self) -> Vec<Point<T>> {
        self.coords.values().cloned().collect()
    }


    pub fn first_outer_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![
            C::SWH,
            C::UL4,
            C::SWG2,
            C::UL2,
            C::SWG1,
            C::UL1,
            C::WH1,
            C::DL1,
            C::NWG1,
            C::DL2,
            C::NWG2,
            C::DL5,
            C::NWH,
        ];
        self.reflected_and_closed_path(west_path, Some(C::UT1), Some(C::DT1))
    }

    pub fn first_inner_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![C::SWH, C::WH1, C::NWH];
        self.reflected_and_closed_path(west_path, None, None)
    }

    pub fn second_outer_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![
            C::WI3,
            C::SWG2,
            C::SWG3,
            C::SWG1,
            C::WI2,
            C::NWG1,
            C::NWG3,
            C::NWG2,
            C::WI1,
        ];
        self.reflected_and_closed_path(west_path, Some(C::UT2), Some(C::DT2))
    }

    pub fn second_inner_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![C::WI3, C::WI2, C::WI1];
        self.reflected_and_closed_path(west_path, None, None)
    }

    pub fn third_outer_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![
            C::SWF,
            C::SWG3,
            C::SWG4,
            C::UL3,
            C::WK,
            C::DL3,
            C::NWG4,
            C::NWG3,
            C::NWF,
        ];
        self.reflected_and_closed_path(west_path, Some(C::UT3), Some(C::DT3))
    }

    pub fn third_inner_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![C::SWF, C::WK, C::NWF];
        self.reflected_and_closed_path(west_path, None, None)
    }

    pub fn fourth_outer_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![C::WJ3, C::SWG4, C::WJ2, C::DL4, C::WG, C::NWG4, C::WJ1];
        self.reflected_and_closed_path(west_path, Some(C::DM1), Some(C::UM1))
    }

    pub fn fourth_inner_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![C::WJ3, C::WJ2, C::WJ1];
        self.reflected_and_closed_path(west_path, None, None)
    }

    pub fn fifth_outer_path(&self) -> Vec<Point<T>> {
        type C = CoordName;
        let west_path = vec![C::WG];
        self.reflected_and_closed_path(west_path, None, Some(C::UM3))
    }

    /// West path must be from bottoms up (excluding the ends)
    fn reflected_and_closed_path(
        &self,
        west_path_coords: Vec<CoordName>,
        top_coord: Option<CoordName>,
        bottom_coord: Option<CoordName>,
    ) -> Vec<Point<T>> {
        let mut west_path: Vec<Point<T>> = west_path_coords
            .into_iter()
            .map(|coord| self.get_point(coord))
            .collect();
        let reflect_y = AffineTransform::scale(-1.0, 1.0, self.center);
        let mut east_path: Vec<Point<T>> = west_path
            .clone()
            .into_iter()
            .map(|point| self.transform(point, &reflect_y))
            .rev()
            .collect();
        if let Some(coord) = top_coord {
            west_path.push(self.get_point(coord));
        };
        if let Some(coord) = bottom_coord {
            east_path.push(self.get_point(coord));
        };

        west_path.append(&mut east_path);
        west_path
    }

    #[allow(dead_code)]
    fn remove_a_point(&mut self, coord: CoordName) {
        self.coords.remove(&coord);
    }

    fn add_bindu(&mut self) {
        self.insert(CoordName::BINDU, self.center)
    }

    fn mid_point(&self, coord1: CoordName, coord2: CoordName) -> Point<T> {
        (self.get_point(coord1) + self.get_point(coord2)) / 2.0
    }

    fn interpolate_and_intersect_with_chord(
        &mut self,
        chord_point: CoordName,
        coord1: CoordName,
        coord2: CoordName,
    ) -> Point<T> {
        #[allow(non_snake_case)]
        let INTERPOLATION_FACTOR = 25.0;
        let point1 = self.get_point(coord1);
        let point2 = self.get_point(coord2);
        let chord_point = self.get_point(chord_point);
        let chord_end = Point::new(
            self.radius * (chord_point.y() / self.radius).acos().sin(),
            chord_point.y(),
        );
        intersection(
            point1 + (point1 - point2) * INTERPOLATION_FACTOR,
            point2 + (point2 - point1) * INTERPOLATION_FACTOR,
            chord_point + (chord_point - chord_end) * INTERPOLATION_FACTOR,
            chord_end + (chord_end - chord_point) * INTERPOLATION_FACTOR,
        )
    }

    fn interpolate_and_intersect(
        &mut self,
        coord1: CoordName,
        coord2: CoordName,
        coord3: CoordName,
        coord4: CoordName,
    ) -> Point<T> {
        #[allow(non_snake_case)]
        let INTERPOLATION_FACTOR = 25.0;
        let point1 = self.get_point(coord1);
        let point2 = self.get_point(coord2);
        let point3 = self.get_point(coord3);
        let point4 = self.get_point(coord4);
        // let points = coords.iter().map(|coord| self.get_point(*coord)).collect();
        intersection(
            point1 + (point1 - point2) * INTERPOLATION_FACTOR,
            point2 + (point2 - point1) * INTERPOLATION_FACTOR,
            point3 + (point3 - point4) * INTERPOLATION_FACTOR,
            point4 + (point4 - point3) * INTERPOLATION_FACTOR,
        )
    }

    fn get_point(&self, coord_name: CoordName) -> Point<T> {
        *self.coords.get(&coord_name).unwrap()
    }

    fn insert(&mut self, coord_name: CoordName, point: Point<T>) {
        self.coords.insert(coord_name, point);
    }

    fn insert_up_down(
        &mut self,
        coord_name_up: CoordName,
        coord_name_down: CoordName,
        up_point: Point<T>,
    ) {
        let reflect_x = AffineTransform::scale(1.0, -1.0, self.center);
        self.coords.insert(coord_name_up, up_point);
        self.coords
            .insert(coord_name_down, self.transform(up_point, &reflect_x));
    }

    fn insert_east_west(
        &mut self,
        coord_name_left: CoordName,
        coord_name_right: CoordName,
        left_point: Point<T>,
    ) {
        let reflect_y = AffineTransform::scale(-1.0, 1.0, self.center);
        self.coords.insert(coord_name_left, left_point);
        self.coords
            .insert(coord_name_right, self.transform(left_point, &reflect_y));
    }

    fn transform(&self, point: Point<T>, transform: &AffineTransform<T>) -> Point<T> {
        self.center + (point - self.center).affine_transform(transform)
    }

    pub fn construct_all_points(&mut self) {
        type C = CoordName;

        // Get the first up and down triangles and their intersections
        // This uses up params g and c.
        let offset_y = self.param_g - self.radius;
        let offset_x = self.radius * (offset_y / self.radius).acos().sin();

        self.insert(C::UM1, Point::new(0, self.radius - self.param_g));
        self.insert_up_down(C::UT1, C::DT1, Point::new(0.0, self.radius));
        self.insert_east_west(C::UL1, C::UR1, Point::new(-offset_x, -offset_y));

        let doffset_y = self.radius - self.param_c;
        let doffset_x = self.radius * (doffset_y / self.radius).acos().sin();
        self.insert(C::DM1, Point::new(0, doffset_y));
        self.insert_east_west(C::DL1, C::DR1, Point::new(-doffset_x, doffset_y));

        let nwg_1 = self.interpolate_and_intersect(C::UT1, C::UL1, C::DL1, C::DR1);
        self.insert_east_west(C::NWG1, C::NEG1, nwg_1);

        let swg_1 = self.interpolate_and_intersect(C::DT1, C::DL1, C::UL1, C::UR1);
        self.insert_east_west(C::SWG1, C::SEG1, swg_1);

        // Get the double intersection on the first path
        let wh = self.interpolate_and_intersect(C::UT1, C::UL1, C::DL1, C::DT1);
        self.insert_east_west(C::WH1, C::EH1, wh);

        // Get the tip of the second down triangle using param i
        self.insert(C::DT2, Point::new(0.0, self.radius - self.param_i));

        // Get the southernmost triple points on the first and the second paths
        let swg_3 = self.interpolate_and_intersect(C::NWG1, C::DT2, C::UL1, C::UR1);
        self.insert_east_west(C::SWG3, C::SEG3, swg_3);

        let swg_2 = self.interpolate_and_intersect(C::DM1, C::SWG3, C::DL1, C::DT1);
        self.insert_east_west(C::SWG2, C::SEG2, swg_2);

        // Get the base of the fourth up triangle
        let ul_4 = self.interpolate_and_intersect_with_chord(C::DT2, C::DM1, C::SWG3);
        self.insert_east_west(C::UL4, C::UR4, ul_4);

        // Use param a to get the ip of the second up triangle
        self.insert(C::UT2, Point::new(0.0, self.radius - self.param_a));

        // Get the remaining triple point on the second path
        let nwg_3 = self.interpolate_and_intersect(C::SWG1, C::UT2, C::DL1, C::DR1);
        self.insert_east_west(C::NWG3, C::NEG3, nwg_3);

        // Use param f to get UM3
        let um_3 = Point::new(0.0, self.radius - self.param_f);
        self.insert(C::UM3, um_3);

        // Get the remaining triple intersection on the first triangle
        let nwg_2 = self.interpolate_and_intersect(C::UM3, C::NWG3, C::UL1, C::UT1);
        self.insert_east_west(C::NWG2, C::NEG2, nwg_2);

        // Get remaining points on the first path
        let dl_5 = self.interpolate_and_intersect_with_chord(C::UT2, C::UM3, C::NWG3);
        self.insert_east_west(C::DL5, C::DR5, dl_5);

        let ul_2 = self.interpolate_and_intersect(C::UT2, C::SWG1, C::SWG2, C::SEG2);
        self.insert_east_west(C::UL2, C::UR2, ul_2);

        let dl_2 = self.interpolate_and_intersect(C::DT2, C::NWG1, C::NWG2, C::NEG2);
        self.insert_east_west(C::DL2, C::DR2, dl_2);

        let nwh = self.interpolate_and_intersect_with_chord(C::UT2, C::UT1, C::UL1);
        self.insert_east_west(C::NWH, C::NEH, nwh);

        let swh = self.interpolate_and_intersect_with_chord(C::DT2, C::DT1, C::DL1);
        self.insert_east_west(C::SWH, C::SEH, swh);

        // Get remaining points on the second path
        let wi_2 = self.interpolate_and_intersect(C::UT2, C::SWG1, C::DT2, C::NWG1);
        self.insert_east_west(C::WI2, C::EI2, wi_2);

        let wi_1 = self.interpolate_and_intersect(C::UT2, C::NWG3, C::NWG2, C::NEG2);
        self.insert_east_west(C::WI1, C::EI1, wi_1);

        let wi_3 = self.interpolate_and_intersect(C::DT2, C::SWG3, C::SWG2, C::SEG2);
        self.insert_east_west(C::WI3, C::EI3, wi_3);

        //  Get the third up triangle
        self.insert(C::UT3, self.mid_point(C::WI1, C::EI1));

        let ul_3 = self.interpolate_and_intersect_with_chord(C::UM3, C::NWG1, C::DT2);
        self.insert_east_west(C::UL3, C::UR3, ul_3);

        // Get the last triple points and the third down triangle
        let wg = self.interpolate_and_intersect(C::UM3, C::NWG3, C::DM1, C::SWG3);
        self.insert_east_west(C::WG, C::EG, wg);

        let swg_4 = self.interpolate_and_intersect_with_chord(C::UM3, C::DM1, C::SWG2);
        self.insert_east_west(C::SWG4, C::SEG4, swg_4);

        self.insert(C::DT3, self.mid_point(C::WI3, C::EI3));
        let dl_3 = self.interpolate_and_intersect(C::DT3, C::SWG4, C::UT2, C::SWG1);
        self.insert_east_west(C::DL3, C::DR3, dl_3);

        let nwg_4 = self.interpolate_and_intersect(C::UT3, C::UL3, C::DL3, C::DR3);
        self.insert_east_west(C::NWG4, C::NEG4, nwg_4);

        // Get the remaining double points on the third path
        let nwf = self.interpolate_and_intersect_with_chord(C::DM1, C::UT3, C::UL3);
        self.insert_east_west(C::NWF, C::NEF, nwf);

        let swf = self.interpolate_and_intersect_with_chord(C::UM1, C::DT3, C::DL3);
        self.insert_east_west(C::SWF, C::SEF, swf);

        let wk = self.interpolate_and_intersect(C::UT3, C::UL3, C::DT3, C::DL3);
        self.insert_east_west(C::WK, C::EK, wk);

        // Get the fourth down triangle
        let dl_4 = self.interpolate_and_intersect_with_chord(C::EG, C::UT3, C::UL3);
        self.insert_east_west(C::DL4, C::DR4, dl_4);

        // Get the remining points on the fourth path
        let wj_1 = self.interpolate_and_intersect_with_chord(C::DL3, C::DM1, C::SWG2);
        self.insert_east_west(C::WJ1, C::EJ1, wj_1);

        let wj_2 = self.interpolate_and_intersect(C::UM1, C::DL4, C::DM1, C::SWG2);
        self.insert_east_west(C::WJ2, C::EJ2, wj_2);

        let wj_3 = self.interpolate_and_intersect_with_chord(C::UM3, C::UM1, C::DL4);
        self.insert_east_west(C::WJ3, C::EJ3, wj_3);

        // Finally, set the bindu
        self.add_bindu();
    }
}
