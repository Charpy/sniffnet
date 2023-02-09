use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::overlay::MyOverlay;
use crate::gui::components::radio::{
    language_radios, sound_bytes_threshold_radios, sound_favorite_radios,
    sound_packets_threshold_radios,
};
use crate::gui::components::tab::get_settings_tabs;
use crate::structs::notifications::{BytesNotification, FavoriteNotification, PacketsNotification};
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{
    get_font, get_font_headers, DEEP_SEA, FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE,
    ICONS, MON_AMOUR, YETI_DAY, YETI_NIGHT,
};
use crate::utility::translations::{
    appearance_title_translation, bytes_threshold_translation, deep_sea_translation,
    favorite_notification_translation, hide_translation, languages_title_translation,
    mon_amour_translation, notifications_title_translation, packets_threshold_translation,
    per_second_translation, settings_translation, specify_multiples_translation,
    threshold_translation, volume_translation, yeti_day_translation, yeti_night_translation,
};
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Language, Sniffer, StyleType};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, horizontal_space, image::Handle, vertical_space, Button, Checkbox, Column, Container,
    Image, Row, Scrollable, Text, TextInput, Tooltip,
};
use iced::Length::Units;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;
use iced_native::widget::Slider;

pub fn settings_notifications_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let mut content = Column::new()
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                MyOverlay::SettingsNotifications,
                MyOverlay::SettingsAppearance,
                MyOverlay::SettingsLanguage,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::TickInit,
                Message::ShowModal(MyOverlay::SettingsAppearance),
                Message::ShowModal(MyOverlay::SettingsLanguage),
            ],
            MyOverlay::SettingsNotifications,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Units(15)))
        .push(
            notifications_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .push(vertical_space(Units(5)));

    let volume_notification_col = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(volume_slider(
            sniffer.language,
            sniffer.style,
            sniffer.notifications.volume,
        ))
        .push(
            Scrollable::new(
                Column::new()
                    .width(Units(720))
                    .push(get_packets_notify(
                        sniffer.notifications.packets_notification,
                        sniffer.language,
                        sniffer.style,
                    ))
                    .push(get_bytes_notify(
                        sniffer.notifications.bytes_notification,
                        sniffer.language,
                        sniffer.style,
                    ))
                    .push(get_favorite_notify(
                        sniffer.notifications.favorite_notification,
                        sniffer.language,
                        sniffer.style,
                    )),
            )
            .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        );

    content = content.push(volume_notification_col);

    Container::new(content)
        .height(Units(400))
        .width(Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

pub fn settings_appearance_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                MyOverlay::SettingsNotifications,
                MyOverlay::SettingsAppearance,
                MyOverlay::SettingsLanguage,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::ShowModal(MyOverlay::SettingsNotifications),
                Message::TickInit,
                Message::ShowModal(MyOverlay::SettingsLanguage),
            ],
            MyOverlay::SettingsAppearance,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Units(15)))
        .push(
            appearance_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Units(10)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    YETI_NIGHT,
                    "Yeti Night".to_string(),
                    yeti_night_translation(sniffer.language).to_string(),
                    Night,
                ))
                .push(horizontal_space(Units(33)))
                .push(get_palette_container(
                    sniffer.style,
                    YETI_DAY,
                    "Yeti Day".to_string(),
                    yeti_day_translation(sniffer.language).to_string(),
                    Day,
                )),
        )
        .push(vertical_space(Units(10)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    DEEP_SEA,
                    "Deep Sea".to_string(),
                    deep_sea_translation(sniffer.language).to_string(),
                    DeepSea,
                ))
                .push(horizontal_space(Units(33)))
                .push(get_palette_container(
                    sniffer.style,
                    MON_AMOUR,
                    "Mon Amour".to_string(),
                    mon_amour_translation(sniffer.language).to_string(),
                    MonAmour,
                )),
        );

    Container::new(content)
        .height(Units(400))
        .width(Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

pub fn settings_language_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let language_active = sniffer.language;
    let col_language_radio = language_radios(language_active, font, sniffer.style);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                MyOverlay::SettingsNotifications,
                MyOverlay::SettingsAppearance,
                MyOverlay::SettingsLanguage,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::ShowModal(MyOverlay::SettingsNotifications),
                Message::ShowModal(MyOverlay::SettingsAppearance),
                Message::TickInit,
            ],
            MyOverlay::SettingsLanguage,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Units(15)))
        .push(
            languages_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Units(20)))
        .push(col_language_radio)
        .push(vertical_space(Units(30)))
        .push(Container::new(Text::new("Support for more languages will come with the next releases.\n\n\
        If you want to help me translating the app in your native language, give a look at Sniffnet issues on GitHub.")
            .width(Length::Units(450))
            .font(font)).padding(10).style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::BorderedRound),
        )));

    Container::new(content)
        .height(Units(400))
        .width(Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn get_packets_notify(
    packets_notification: PacketsNotification,
    language: Language,
    style: StyleType,
) -> Column<'static, Message> {
    let checkbox = Checkbox::new(
        packets_threshold_translation(language),
        packets_notification.threshold.is_some(),
        move |toggled| {
            if toggled {
                Message::UpdatePacketsNotification(
                    PacketsNotification {
                        threshold: Some(packets_notification.previous_threshold),
                        ..packets_notification
                    },
                    false,
                )
            } else {
                Message::UpdatePacketsNotification(
                    PacketsNotification {
                        threshold: None,
                        ..packets_notification
                    },
                    false,
                )
            }
        },
    )
    .size(18)
    .font(get_font(style))
    .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
        StyleTuple(style, ElementType::Standard),
    ));

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if packets_notification.threshold.is_none() {
        Column::new()
            .padding(5)
            .push(Container::new(ret_val).padding(10).width(Units(700)).style(
                <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
                    style,
                    ElementType::BorderedRound,
                )),
            ))
    } else {
        let input_row = Row::new()
            .push(horizontal_space(Units(50)))
            .push(Text::new(threshold_translation(language)).font(get_font(style)))
            .push(input_group_packets(packets_notification, style, language));
        let sound_row =
            Row::new()
                .push(horizontal_space(Units(50)))
                .push(sound_packets_threshold_radios(
                    packets_notification,
                    get_font(style),
                    style,
                    language,
                ));
        ret_val = ret_val
            .push(vertical_space(Units(5)))
            .push(input_row)
            .push(sound_row);
        Column::new()
            .padding(5)
            .push(Container::new(ret_val).padding(10).width(Units(700)).style(
                <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
                    style,
                    ElementType::BorderedRound,
                )),
            ))
    }
}

fn get_bytes_notify(
    bytes_notification: BytesNotification,
    language: Language,
    style: StyleType,
) -> Column<'static, Message> {
    let checkbox = Checkbox::new(
        bytes_threshold_translation(language),
        bytes_notification.threshold.is_some(),
        move |toggled| {
            if toggled {
                Message::UpdateBytesNotification(
                    BytesNotification {
                        threshold: Some(bytes_notification.previous_threshold),
                        ..bytes_notification
                    },
                    false,
                )
            } else {
                Message::UpdateBytesNotification(
                    BytesNotification {
                        threshold: None,
                        ..bytes_notification
                    },
                    false,
                )
            }
        },
    )
    .size(18)
    .font(get_font(style))
    .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
        StyleTuple(style, ElementType::Standard),
    ));

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if bytes_notification.threshold.is_none() {
        Column::new()
            .padding(5)
            .push(Container::new(ret_val).padding(10).width(Units(700)).style(
                <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
                    style,
                    ElementType::BorderedRound,
                )),
            ))
    } else {
        let input_row = Row::new()
            .push(horizontal_space(Units(50)))
            .push(Text::new(threshold_translation(language)).font(get_font(style)))
            .push(input_group_bytes(bytes_notification, style, language));
        let sound_row =
            Row::new()
                .push(horizontal_space(Units(50)))
                .push(sound_bytes_threshold_radios(
                    bytes_notification,
                    get_font(style),
                    style,
                    language,
                ));
        ret_val = ret_val
            .push(vertical_space(Units(5)))
            .push(input_row)
            .push(sound_row);
        Column::new()
            .padding(5)
            .push(Container::new(ret_val).padding(10).width(Units(700)).style(
                <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
                    style,
                    ElementType::BorderedRound,
                )),
            ))
    }
}

fn get_favorite_notify(
    favorite_notification: FavoriteNotification,
    language: Language,
    style: StyleType,
) -> Column<'static, Message> {
    let checkbox = Checkbox::new(
        favorite_notification_translation(language),
        favorite_notification.notify_on_favorite,
        move |toggled| {
            Message::UpdateFavoriteNotification(
                if toggled {
                    FavoriteNotification::on(favorite_notification.sound)
                } else {
                    FavoriteNotification::off(favorite_notification.sound)
                },
                false,
            )
        },
    )
    .size(18)
    .font(get_font(style))
    .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
        StyleTuple(style, ElementType::Standard),
    ));

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if favorite_notification.notify_on_favorite {
        let sound_row = Row::new()
            .push(horizontal_space(Units(50)))
            .push(sound_favorite_radios(
                favorite_notification,
                get_font(style),
                style,
                language,
            ));
        ret_val = ret_val.push(vertical_space(Units(5))).push(sound_row);
        Column::new()
            .padding(5)
            .push(Container::new(ret_val).padding(10).width(Units(700)).style(
                <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
                    style,
                    ElementType::BorderedRound,
                )),
            ))
    } else {
        Column::new()
            .padding(5)
            .push(Container::new(ret_val).padding(10).width(Units(700)).style(
                <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
                    style,
                    ElementType::BorderedRound,
                )),
            ))
    }
}

fn input_group_packets(
    packets_notification: PacketsNotification,
    style: StyleType,
    language: Language,
) -> Container<'static, Message> {
    let curr_threshold_str = &packets_notification.threshold.unwrap().to_string();
    let input_row = Row::new()
        .spacing(10)
        .push(
            TextInput::new(
                "0",
                if curr_threshold_str == "0" {
                    ""
                } else {
                    curr_threshold_str
                },
                move |value| {
                    let packets_notification =
                        PacketsNotification::from(&value, Some(packets_notification));
                    Message::UpdatePacketsNotification(packets_notification, false)
                },
            )
            .padding(1)
            .font(get_font(style))
            .width(Length::Units(100))
            .style(<StyleTuple as Into<iced::theme::TextInput>>::into(
                StyleTuple(style, ElementType::Standard),
            )),
        )
        .push(
            Text::new(per_second_translation(language))
                .font(get_font(style))
                .vertical_alignment(Vertical::Center)
                .size(FONT_SIZE_FOOTER),
        );
    Container::new(input_row)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn input_group_bytes(
    bytes_notification: BytesNotification,
    style: StyleType,
    language: Language,
) -> Container<'static, Message> {
    let mut info_str = per_second_translation(language).to_string();
    info_str.push_str(specify_multiples_translation(language));
    let mut curr_threshold_str = (bytes_notification.threshold.unwrap()
        / bytes_notification.byte_multiple.get_multiplier())
    .to_string();
    curr_threshold_str.push_str(bytes_notification.byte_multiple.get_char());
    let input_row = Row::new()
        .spacing(10)
        .push(
            TextInput::new(
                "0",
                if curr_threshold_str == "0" {
                    ""
                } else {
                    &curr_threshold_str
                },
                move |value| {
                    let bytes_notification =
                        BytesNotification::from(&value, Some(bytes_notification));
                    Message::UpdateBytesNotification(bytes_notification, false)
                },
            )
            .padding(1)
            .font(get_font(style))
            .width(Length::Units(100))
            .style(<StyleTuple as Into<iced::theme::TextInput>>::into(
                StyleTuple(style, ElementType::Standard),
            )),
        )
        .push(
            Text::new(info_str)
                .font(get_font(style))
                .vertical_alignment(Vertical::Center)
                .size(FONT_SIZE_FOOTER),
        );
    Container::new(input_row)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn volume_slider(language: Language, style: StyleType, volume: u8) -> Container<'static, Message> {
    Container::new(
        Column::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(Text::new(volume_translation(language, volume)).font(get_font(style)))
            .push(
                Row::new()
                    .push(
                        Text::new('Y'.to_string())
                            .width(Units(30))
                            .vertical_alignment(Vertical::Center)
                            .size(20)
                            .font(ICONS),
                    )
                    .push(
                        Slider::new(0..=100, volume, Message::ChangeVolume)
                            .step(5)
                            .width(Units(200))
                            .style(<StyleTuple as Into<iced::theme::Slider>>::into(StyleTuple(
                                style,
                                ElementType::Standard,
                            ))),
                    )
                    .push(horizontal_space(Length::Units(10)))
                    .push(
                        Text::new('Z'.to_string())
                            .vertical_alignment(Vertical::Center)
                            .size(20)
                            .font(ICONS),
                    ),
            ),
    )
    .padding(5)
    .width(Length::Fill)
    .height(Length::Units(60))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

fn get_palette_container(
    style: StyleType,
    picture: &[u8],
    name: String,
    description: String,
    on_press: StyleType,
) -> Button<'static, Message> {
    let font = get_font(style);
    let content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(Image::new(Handle::from_memory(Vec::from(picture))).width(Units(300)))
        .push(Text::new(description).font(font));

    Button::new(content)
        .height(Units(130))
        .width(Units(350))
        .padding(10)
        .style(StyleTuple(style, ElementType::BorderedRound).into())
        .on_press(Message::Style(on_press))
}

fn get_settings_header(style: StyleType, language: Language) -> Container<'static, Message> {
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new(settings_translation(language))
                    .font(get_font_headers(style))
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    Tooltip::new(
                        button(
                            Text::new("x")
                                .font(get_font(style))
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Units(20))
                        .width(Units(20))
                        .style(StyleTuple(style, ElementType::Standard).into())
                        .on_press(Message::HideModal(false)),
                        hide_translation(language),
                        Position::Right,
                    )
                    .font(get_font(style))
                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                        StyleTuple(style, ElementType::Tooltip),
                    )),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Units(40))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}
