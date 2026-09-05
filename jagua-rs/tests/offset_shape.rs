use jagua_rs::geometry::primitives::{Point, SPolygon};
use jagua_rs::geometry::shape_modification::{
    ShapeModifyMode, offset_shape, shape_modification_valid,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct OffsetFixture {
    spacing_mm: f32,
    outer: Vec<[f64; 2]>,
}

#[test]
fn dense_raster_inflation_produces_a_valid_conservative_polygon() {
    let fixture: OffsetFixture = serde_json::from_str(include_str!(
        "fixtures/adr0017-spacing-proxy-unavailable.json"
    ))
    .expect("ADR 0017 offset fixture should parse");
    let source = SPolygon::new(
        fixture
            .outer
            .into_iter()
            .map(|[x, y]| Point(x as f32, y as f32))
            .collect(),
    )
    .expect("fixture source polygon should be valid");

    let inflated = offset_shape(&source, ShapeModifyMode::Inflate, fixture.spacing_mm / 2.0)
        .expect("dense raster offset should remain a simple polygon");

    assert!(inflated.area > source.area);
    let offset = fixture.spacing_mm / 2.0;
    let tolerance = 1.0e-3;
    assert!(
        inflated.bbox.x_min <= source.bbox.x_min - offset + tolerance,
        "x_min: source={}, inflated={}, expected={}",
        source.bbox.x_min,
        inflated.bbox.x_min,
        source.bbox.x_min - offset
    );
    assert!(
        inflated.bbox.y_min <= source.bbox.y_min - offset + tolerance,
        "y_min: source={}, inflated={}, expected={}",
        source.bbox.y_min,
        inflated.bbox.y_min,
        source.bbox.y_min - offset
    );
    assert!(
        inflated.bbox.x_max >= source.bbox.x_max + offset - tolerance,
        "x_max: source={}, inflated={}, expected={}",
        source.bbox.x_max,
        inflated.bbox.x_max,
        source.bbox.x_max + offset
    );
    assert!(
        inflated.bbox.y_max >= source.bbox.y_max + offset - tolerance,
        "y_max: source={}, inflated={}, expected={}",
        source.bbox.y_max,
        inflated.bbox.y_max,
        source.bbox.y_max + offset
    );
    assert!(shape_modification_valid(
        &source,
        &inflated,
        ShapeModifyMode::Inflate
    ));
}
