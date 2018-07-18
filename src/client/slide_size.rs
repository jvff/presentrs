use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct SlideMeasure {
    container_width: f64,
    container_height: f64,
    width: f64,
    height: f64,
    scale: f64,
}

impl SlideMeasure {
    pub fn measure_to_fit(
        slide: &SlideSize,
        container_width: f64,
        container_height: f64,
    ) -> Self {
        let scale_x = container_width / slide.original_width;
        let scale_y = container_height / slide.original_height;

        let scale = scale_x.min(scale_y);

        let width = slide.original_width * scale;
        let height = slide.original_height * scale;

        SlideMeasure {
            container_width,
            container_height,
            width,
            height,
            scale,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlideSize {
    original_width: f64,
    original_height: f64,
    scale: f64,
    translate_x: f64,
    translate_y: f64,
    margin_top: f64,
    margin_bottom: f64,
    margin_left: f64,
    margin_right: f64,
}

impl Default for SlideSize {
    fn default() -> Self {
        SlideSize::new(1.0, 1.0)
    }
}

impl SlideSize {
    pub fn new(width: f64, height: f64) -> Self {
        SlideSize {
            original_width: width,
            original_height: height,
            scale: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
            margin_top: 0.0,
            margin_bottom: 0.0,
            margin_left: 0.0,
            margin_right: 0.0,
        }
    }

    pub fn measure_to_fit_in(&self, width: f64, height: f64) -> SlideMeasure {
        SlideMeasure::measure_to_fit(&self, width, height)
    }

    pub fn resize_to_fit_in(&mut self, width: f64, height: f64) {
        let measure = self.measure_to_fit_in(width, height);

        self.resize(measure);
    }

    pub fn resize(&mut self, measure: SlideMeasure) {
        self.scale = measure.scale;

        let delta_x = measure.width - self.original_width;
        let delta_y = measure.height - self.original_height;

        self.translate_x = delta_x / 2.0;
        self.translate_y = delta_y / 2.0;

        let horizontal_margin = (measure.container_width - measure.width) / 2.0;
        let vertical_margin = (measure.container_height - measure.height) / 2.0;

        self.margin_left = horizontal_margin;
        self.margin_right = horizontal_margin;
        self.margin_top = vertical_margin;
        self.margin_bottom = vertical_margin;
    }
}

impl Display for SlideSize {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, " width: {}px;", self.original_width)?;
        write!(formatter, " height: {}px;", self.original_height)?;
        write!(
            formatter,
            " transform: translate({}px, {}px) scale({});",
            self.translate_x, self.translate_y, self.scale,
        )?;
        write!(formatter, " margin-top: {}px;", self.margin_top)?;
        write!(formatter, " margin-bottom: {}px;", self.margin_bottom)?;
        write!(formatter, " margin-left: {}px;", self.margin_left)?;
        write!(formatter, " margin-right: {}px;", self.margin_right)?;

        Ok(())
    }
}
