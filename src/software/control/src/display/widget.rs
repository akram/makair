// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::widget::Id as WidgetId;
use conrod_core::{
    color::{self, Color},
    widget::{
        self, canvas, id::List, primitive::shape::Style, rounded_rectangle::RoundedRectangle,
    },
    Colorable, Positionable, Sizeable, Widget,
};

use telemetry::alarm::AlarmCode;
use telemetry::structures::AlarmPriority;

use crate::config::environment::*;

use super::fonts::Fonts;

pub type WidgetIds = (WidgetId, WidgetId, WidgetId, WidgetId, WidgetId);

pub struct BackgroundWidgetConfig {
    color: conrod_core::color::Color,
    id: WidgetId,
}

impl BackgroundWidgetConfig {
    pub fn new(color: conrod_core::color::Color, id: WidgetId) -> BackgroundWidgetConfig {
        BackgroundWidgetConfig { color, id }
    }
}

pub struct BrandingWidgetConfig<'a> {
    version_firmware: &'a str,
    version_control: &'a str,
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
    pub ids: (WidgetId, WidgetId),
}

pub struct TelemetryWidgetConfig<'a> {
    pub title: &'a str,
    pub value: String,
    pub unit: &'a str,
    pub ids: WidgetIds,
    pub x_position: f64,
    pub y_position: f64,
    pub background_color: Color,
}

pub struct GraphWidgetConfig {
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
    parent: WidgetId,
    id: WidgetId,
}

impl<'a> BrandingWidgetConfig<'a> {
    pub fn new(
        version_firmware: &'a str,
        version_control: &'a str,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        ids: (WidgetId, WidgetId),
    ) -> BrandingWidgetConfig<'a> {
        BrandingWidgetConfig {
            version_firmware,
            version_control,
            width,
            height,
            image,
            ids,
        }
    }
}

impl GraphWidgetConfig {
    pub fn new(
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        parent: WidgetId,
        id: WidgetId,
    ) -> GraphWidgetConfig {
        GraphWidgetConfig {
            width,
            height,
            image,
            parent,
            id,
        }
    }
}

pub struct ErrorWidgetConfig {
    error: String,
    id: WidgetId,
}

impl ErrorWidgetConfig {
    pub fn new(error: String, id: WidgetId) -> ErrorWidgetConfig {
        ErrorWidgetConfig { error, id }
    }
}

pub struct StopWidgetConfig {
    pub parent: WidgetId,
    pub background: WidgetId,
    pub container_borders: WidgetId,
    pub container: WidgetId,
    pub title: WidgetId,
    pub message: WidgetId,
}

pub struct NoDataWidgetConfig {
    id: WidgetId,
}

impl NoDataWidgetConfig {
    pub fn new(id: WidgetId) -> NoDataWidgetConfig {
        NoDataWidgetConfig { id }
    }
}

pub struct InitializingWidgetConfig {
    id: WidgetId,
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
}

impl InitializingWidgetConfig {
    pub fn new(
        id: WidgetId,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
    ) -> InitializingWidgetConfig {
        InitializingWidgetConfig {
            id,
            width,
            height,
            image,
        }
    }
}

pub struct AlarmsWidgetConfig<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,
    pub title: WidgetId,
    pub empty: WidgetId,
    pub alarm_widgets: &'a List,
    pub alarm_codes_containers: &'a List,
    pub alarm_codes: &'a List,
    pub alarm_messages_containers: &'a List,
    pub alarm_messages: &'a List,
    pub alarms: &'a [(&'a AlarmCode, &'a AlarmPriority)],
}

pub enum ControlWidgetType<'a> {
    Alarms(AlarmsWidgetConfig<'a>),
    Background(BackgroundWidgetConfig),
    Error(ErrorWidgetConfig),
    Branding(BrandingWidgetConfig<'a>),
    Initializing(InitializingWidgetConfig),
    Graph(GraphWidgetConfig),
    NoData(NoDataWidgetConfig),
    Stop(StopWidgetConfig),
    Telemetry(TelemetryWidgetConfig<'a>),
}

pub struct ControlWidget<'a> {
    ui: conrod_core::UiCell<'a>,
    fonts: &'a Fonts,
}

impl<'a> ControlWidget<'a> {
    pub fn new(ui: conrod_core::UiCell<'a>, fonts: &'a Fonts) -> ControlWidget<'a> {
        ControlWidget { ui, fonts }
    }

    pub fn render(&mut self, widget_type: ControlWidgetType<'a>) -> f64 {
        match widget_type {
            ControlWidgetType::Alarms(config) => self.alarms(config),
            ControlWidgetType::Background(config) => self.background(config),
            ControlWidgetType::Error(config) => self.error(config),
            ControlWidgetType::Branding(config) => self.branding(config),
            ControlWidgetType::Initializing(config) => self.initializing(config),
            ControlWidgetType::Graph(config) => self.graph(config),
            ControlWidgetType::NoData(config) => self.no_data(config),
            ControlWidgetType::Stop(config) => self.stop(config),
            ControlWidgetType::Telemetry(config) => self.telemetry_widget(config),
        }
    }

    fn alarms(&mut self, config: AlarmsWidgetConfig) -> f64 {
        let dimensions = [
            DISPLAY_ALARM_CONTAINER_WIDTH,
            DISPLAY_ALARM_CONTAINER_HEIGHT,
        ];

        // Draw container box
        RoundedRectangle::fill_with(
            dimensions,
            DISPLAY_ROUNDED_RECTANGLES_ROUND,
            DISPLAY_ALARM_CONTAINER_COLOR,
        )
        .mid_top_with_margin_on(config.parent, 10.0)
        .set(config.container, &mut self.ui);

        // Draw text
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(11);

        widget::text::Text::new("ALARMS")
            .with_style(text_style)
            .mid_left_with_margin_on(config.container, DISPLAY_ALARM_CONTAINER_PADDING_LEFT)
            .set(config.title, &mut self.ui);

        // Append all alarms?
        // Notice: only draw alarms box if there are active alarms
        if config.alarms.len() > 0 {
            // TODO: we should not cap the alarms to display in case there are more than 2; this is \
            //   unsafe regulatory and safety wise
            let max_alarms = std::cmp::min(DISPLAY_MAX_ALARMS, config.alarms.len());

            for x in 0..max_alarms {
                let (code, alarm) = config.alarms.get(x).unwrap();

                self.alarm(&config, **code, alarm, x);
            }
        } else {
            widget::text::Text::new("There is no active alarm.")
                .color(Color::Rgba(1.0, 1.0, 1.0, 0.5))
                .font_size(11)
                .right_from(config.title, 42.0)
                .set(config.empty, &mut self.ui);
        }

        0 as _
    }

    // TODO: create a rounded rectangle of either right or left angles
    //   check how rounded rectangles are created
    fn alarm(
        &mut self,
        config: &AlarmsWidgetConfig,
        code: AlarmCode,
        alarm_priority: &AlarmPriority,
        index: usize,
    ) {
        let mut style = canvas::Style::default();

        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);
        style.color = Some(color::TRANSPARENT);

        let from_top = if index == 0 {
            DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
        } else {
            DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
                + index as f64
                    * (DISPLAY_ALARM_MESSAGE_HEIGHT + DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER)
        };

        canvas::Canvas::new()
            .with_style(style)
            .y_place_on(
                config.container,
                conrod_core::position::Place::End(Some(from_top)),
            )
            .right_from(config.title, 28.0)
            .set(config.alarm_widgets[index], &mut self.ui);

        self.alarm_code(&config, code, alarm_priority, index);
        self.alarm_message(&config, code, alarm_priority, index);
    }

    fn alarm_code_color(&self, alarm_priority: &AlarmPriority) -> Color {
        match alarm_priority {
            AlarmPriority::High => Color::Rgba(1.0, 32.0 / 255.0, 32.0 / 255.0, 1.0),
            AlarmPriority::Medium => Color::Rgba(1.0, 138.0 / 255.0, 0.0, 1.0),
            AlarmPriority::Low => Color::Rgba(1.0, 195.0 / 255.0, 0.0, 1.0),
        }
    }

    fn alarm_message_color(&self, alarm_priority: &AlarmPriority) -> Color {
        match alarm_priority {
            AlarmPriority::High => Color::Rgba(169.0 / 255.0, 35.0 / 255.0, 35.0 / 255.0, 1.0),
            AlarmPriority::Medium => Color::Rgba(169.0 / 255.0, 99.0 / 255.0, 16.0 / 255.0, 1.0),
            AlarmPriority::Low => Color::Rgba(174.0 / 255.0, 133.0 / 255.0, 0.0, 1.0),
        }
    }

    fn alarm_code(
        &mut self,
        config: &AlarmsWidgetConfig,
        alarm_code: AlarmCode,
        alarm_priority: &AlarmPriority,
        index: usize,
    ) {
        let color = self.alarm_code_color(alarm_priority);

        // Draw canvas
        let mut style = canvas::Style::default();

        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);
        style.color = Some(color);

        widget::Canvas::new()
            .with_style(style)
            .w_h(DISPLAY_ALARM_CODE_WIDTH, DISPLAY_ALARM_CODE_HEIGHT)
            .x_place_on(
                config.alarm_widgets[index],
                conrod_core::position::Place::Start(None),
            )
            .set(config.alarm_codes_containers[index], &mut self.ui);

        // Draw text
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(11);

        widget::text::Text::new(&format!("{}", alarm_code.code()))
            .with_style(text_style)
            .mid_top_with_margin_on(config.alarm_codes_containers[index], 4.0)
            .set(config.alarm_codes[index], &mut self.ui);
    }

    fn alarm_message(
        &mut self,
        config: &AlarmsWidgetConfig,
        code: AlarmCode,
        alarm_priority: &AlarmPriority,
        index: usize,
    ) {
        let color = self.alarm_message_color(alarm_priority);

        let mut style = canvas::Style::default();

        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);
        style.color = Some(color);

        widget::Canvas::new()
            .with_style(style)
            .w_h(DISPLAY_ALARM_MESSAGE_WIDTH, DISPLAY_ALARM_MESSAGE_HEIGHT)
            .x_place_on(
                config.alarm_widgets[index],
                conrod_core::position::Place::Start(Some(DISPLAY_ALARM_CODE_WIDTH)),
            )
            .set(config.alarm_messages_containers[index], &mut self.ui);

        widget::text::Text::new(&code.description())
            .color(color::WHITE)
            .font_size(10)
            .top_left_with_margins_on(config.alarm_messages_containers[index], 4.0, 10.0)
            .set(config.alarm_messages[index], &mut self.ui);
    }

    fn background(&mut self, config: BackgroundWidgetConfig) -> f64 {
        widget::Canvas::new()
            .color(config.color)
            .set(config.id, &mut self.ui);

        0 as _
    }

    fn branding(&mut self, config: BrandingWidgetConfig) -> f64 {
        // Display branding image
        widget::Image::new(config.image)
            .w_h(config.width, config.height)
            .top_left_with_margins(BRANDING_IMAGE_MARGIN_TOP, BRANDING_IMAGE_MARGIN_LEFT)
            .set(config.ids.0, &mut self.ui);

        // Display branding text
        let branding_text = format!("F{} | C{}", config.version_firmware, config.version_control);

        widget::Text::new(&branding_text)
            .color(color::WHITE.with_alpha(0.45))
            .top_left_with_margins(BRANDING_TEXT_MARGIN_TOP, BRANDING_TEXT_MARGIN_LEFT)
            .font_size(10)
            .set(config.ids.1, &mut self.ui);

        config.width
    }

    fn graph(&mut self, config: GraphWidgetConfig) -> f64 {
        widget::Image::new(config.image)
            .w_h(config.width, config.height)
            .mid_bottom_with_margin_on(config.parent, GRAPH_DRAW_SPACING_FROM_BOTTOM)
            .set(config.id, &mut self.ui);

        config.width
    }

    fn telemetry_widget(&mut self, config: TelemetryWidgetConfig) -> f64 {
        // Create rounded rectangle
        widget::rounded_rectangle::RoundedRectangle::styled(
            [TELEMETRY_WIDGET_SIZE_WIDTH, TELEMETRY_WIDGET_SIZE_HEIGHT],
            2.5,
            widget::primitive::shape::Style::Fill(Some(config.background_color)),
        )
        .bottom_left_with_margins_on(
            config.ids.0,
            config.y_position,
            config.x_position + TELEMETRY_WIDGET_SIZE_SPACING,
        )
        .set(config.ids.1, &mut self.ui);

        // Create each text unit
        widget::Text::new(config.title)
            .color(color::WHITE)
            .top_left_with_margins_on(config.ids.1, 10.0, 20.0)
            .font_size(11)
            .set(config.ids.2, &mut self.ui);

        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(17);

        widget::Text::new(&config.value)
            .with_style(text_style)
            .mid_left_with_margin_on(config.ids.1, 20.0)
            .set(config.ids.3, &mut self.ui);

        widget::Text::new(config.unit)
            .color(color::WHITE.with_alpha(0.2))
            .bottom_left_with_margins_on(config.ids.1, 12.0, 20.0)
            .font_size(11)
            .set(config.ids.4, &mut self.ui);

        TELEMETRY_WIDGET_SIZE_WIDTH
    }

    fn error(&mut self, config: ErrorWidgetConfig) -> f64 {
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(30);

        widget::Text::new(&format!("An error happened:\n{}", config.error)) // using \n instead of the wrap methods because I couldn't make them work
            .color(color::WHITE)
            .align_top() // Aligned to top otherwise I can't make the line breaks work
            .with_style(text_style)
            .set(config.id, &mut self.ui);

        0 as _
    }

    fn stop(&mut self, config: StopWidgetConfig) -> f64 {
        let mut style = canvas::Style::default();

        style.color = Some(Color::Rgba(0.0, 0.0, 0.0, 0.75));
        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);

        canvas::Canvas::new()
            .with_style(style)
            .w_h(
                DISPLAY_WINDOW_SIZE_WIDTH as _,
                DISPLAY_WINDOW_SIZE_HEIGHT as f64 - DISPLAY_ALARM_CONTAINER_HEIGHT,
            )
            .x_y(0.0, -DISPLAY_ALARM_CONTAINER_HEIGHT)
            .set(config.background, &mut self.ui);

        let container_borders_style = Style::Fill(Some(Color::Rgba(
            81.0 / 255.0,
            81.0 / 255.0,
            81.0 / 255.0,
            1.0,
        )));
        RoundedRectangle::styled(
            [
                DISPLAY_STOPPED_MESSAGE_CONTAINER_WIDTH + 5.0,
                DISPLAY_STOPPED_MESSAGE_CONTAINER_HEIGHT + 5.0,
            ],
            DISPLAY_ROUNDED_RECTANGLES_ROUND,
            container_borders_style,
        )
        .middle_of(config.parent)
        .set(config.container_borders, &mut self.ui);

        let mut container_style = canvas::Style::default();
        container_style.color = Some(Color::Rgba(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0, 1.0));
        container_style.border = Some(0.0);
        container_style.border_color = Some(color::TRANSPARENT);

        canvas::Canvas::new()
            .with_style(container_style)
            .w_h(
                DISPLAY_STOPPED_MESSAGE_CONTAINER_WIDTH,
                DISPLAY_STOPPED_MESSAGE_CONTAINER_HEIGHT,
            )
            .middle_of(config.container_borders)
            .set(config.container, &mut self.ui);

        let mut title_style = widget::text::Style::default();
        title_style.color = Some(color::WHITE);
        title_style.font_size = Some(18);
        title_style.font_id = Some(Some(self.fonts.bold));

        widget::text::Text::new("Ventilator unit inactive")
            .with_style(title_style)
            .mid_top_with_margin_on(config.container, DISPLAY_STOPPED_MESSAGE_PADDING_TOP)
            .set(config.title, &mut self.ui);

        let mut message_style = widget::text::Style::default();
        message_style.color = Some(Color::Rgba(1.0, 1.0, 1.0, 0.75));
        message_style.font_size = Some(13);
        message_style.font_id = Some(Some(self.fonts.regular));

        widget::text::Text::new("Please re-enable it to resume respiration")
            .with_style(message_style)
            .mid_bottom_with_margin_on(config.container, DISPLAY_STOPPED_MESSAGE_PADDING_BOTTOM)
            .set(config.message, &mut self.ui);

        0 as _
    }

    fn no_data(&mut self, config: NoDataWidgetConfig) -> f64 {
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(30);

        widget::Text::new("Device disconnected or no data received")
            .color(color::WHITE)
            .middle()
            .with_style(text_style)
            .set(config.id, &mut self.ui);
        0 as _
    }

    fn initializing(&mut self, config: InitializingWidgetConfig) -> f64 {
        widget::Image::new(config.image)
            .w_h(config.width, config.height)
            .middle()
            .set(config.id, &mut self.ui);

        0 as _
    }
}
