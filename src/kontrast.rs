#[derive(Default)]
pub struct KontrastStruct {
    text_color: QColor,
    font_size: i32,
    background_color: QColor,
    grabbed_color: QColor,
}

enum Quality {
    Bad,
    Good,
    Perfect,
}

struct ContrastQualities {
    small: Quality,
    medium: Quality,
    large: Quality,
}

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {

        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(QColor, text_color, READ, WRITE, NOTIFY = text_color_changed)]
        #[qproperty(i32, text_hue, READ = text_hue, WRITE = set_text_hue, NOTIFY = text_color_changed)]
        #[qproperty(i32, text_lightness, READ = text_lightness, WRITE = set_text_lightness, NOTIFY = text_color_changed)]
        #[qproperty(i32, text_saturation, READ = text_saturation, WRITE = set_text_saturation, NOTIFY = text_color_changed)]
        #[qproperty(i32, font_size, READ, WRITE, NOTIFY = font_size_changed)]
        #[qproperty(QColor, background_color, READ, WRITE, NOTIFY = background_color_changed)]
        #[qproperty(i32, background_hue, READ = background_hue, WRITE = set_background_hue, NOTIFY = background_color_changed)]
        #[qproperty(i32, background_saturation, READ = background_saturation, WRITE = set_background_saturation, NOTIFY = background_color_changed)]
        #[qproperty(i32, background_lightness, READ = background_lightness, WRITE = set_background_lightness, NOTIFY = background_color_changed)]
        #[qproperty(QString, font_size_label, READ = font_size_label, NOTIFY = font_size_changed)]
        #[qproperty(f32, contrast, READ = contrast, NOTIFY = contrast_changed)]
        #[qproperty(QColor, display_text_color, READ = display_text_color, NOTIFY = contrast_changed)]
        #[qproperty(QColor, grabbed_color)]
        type Kontrast = super::KontrastStruct;

        // signals

        #[qsignal]
        fn text_color_changed(self: Pin<&mut Kontrast>);

        #[qsignal]
        fn font_size_changed(self: Pin<&mut Kontrast>);

        #[qsignal]
        fn background_color_changed(self: Pin<&mut Kontrast>);

        #[qsignal]
        fn contrast_changed(self: Pin<&mut Kontrast>);

        // properties

        #[qinvokable]
        fn text_hue(self: &Kontrast) -> i32;

        #[qinvokable]
        fn set_text_hue(self: Pin<&mut Kontrast>, hue: i32);

        #[qinvokable]
        fn text_lightness(self: &Kontrast) -> i32;

        #[qinvokable]
        fn set_text_lightness(self: Pin<&mut Kontrast>, lightness: i32);

        #[qinvokable]
        fn text_saturation(self: &Kontrast) -> i32;

        #[qinvokable]
        fn set_text_saturation(self: Pin<&mut Kontrast>, saturation: i32);

        #[qinvokable]
        fn background_hue(self: &Kontrast) -> i32;

        #[qinvokable]
        fn set_background_hue(self: Pin<&mut Kontrast>, hue: i32);

        #[qinvokable]
        fn background_lightness(self: &Kontrast) -> i32;

        #[qinvokable]
        fn set_background_lightness(self: Pin<&mut Kontrast>, lightness: i32);

        #[qinvokable]
        fn background_saturation(self: &Kontrast) -> i32;

        #[qinvokable]
        fn set_background_saturation(self: Pin<&mut Kontrast>, saturation: i32);

        #[qinvokable]
        fn font_size_label(self: &Kontrast) -> QString;

        #[qinvokable]
        fn display_text_color(self: &Kontrast) -> QColor;

        #[qinvokable]
        fn contrast(self: &Kontrast) -> f32;

        #[qinvokable]
        fn grab_color(self: Pin<&mut Kontrast>);

        // methods

        #[qinvokable]
        fn random(self: Pin<&mut Kontrast>);

        #[qinvokable]
        fn reverse(self: Pin<&mut Kontrast>);

    }

    impl cxx_qt::Constructor<()> for Kontrast {}
    impl cxx_qt::Threading for Kontrast {}
}

use std::pin::Pin;

use cxx_kde_frameworks::ki18n::i18n;
use cxx_qt::Threading;
use cxx_qt_lib::{QColor, QString};
use rand::{thread_rng, Rng};

use crate::dbus;

impl ffi::Kontrast {
    fn text_hue(self: &Self) -> i32 {
        return self.text_color.hsl_hue();
    }

    fn set_text_hue(mut self: Pin<&mut Self>, hue: i32) {
        let color = self.as_mut().text_color.to_owned();
        self.as_mut()
            .set_text_color(QColor::from_hsl(hue, color.saturation(), color.lightness()));
    }

    fn text_lightness(self: &Self) -> i32 {
        return self.text_color.lightness();
    }

    fn set_text_lightness(mut self: Pin<&mut Self>, lightness: i32) {
        let color = self.as_mut().text_color.to_owned();
        self.as_mut()
            .set_text_color(QColor::from_hsl(color.hue(), color.saturation(), lightness));
    }

    fn text_saturation(self: &Self) -> i32 {
        return self.text_color.hsl_saturation();
    }

    fn set_text_saturation(mut self: Pin<&mut Self>, saturation: i32) {
        let color = self.as_mut().text_color.to_owned();
        self.as_mut()
            .set_text_color(QColor::from_hsl(color.hue(), saturation, color.lightness()));
    }

    fn background_hue(self: &Self) -> i32 {
        return self.background_color.hsl_hue();
    }

    fn set_background_hue(mut self: Pin<&mut Self>, hue: i32) {
        let color = self.as_mut().background_color.to_owned();
        self.as_mut().set_background_color(QColor::from_hsl(
            hue,
            color.saturation(),
            color.lightness(),
        ));
    }

    fn background_lightness(self: &Self) -> i32 {
        return self.background_color.lightness();
    }

    fn set_background_lightness(mut self: Pin<&mut Self>, lightness: i32) {
        let color = self.as_mut().background_color.to_owned();
        self.as_mut().set_background_color(QColor::from_hsl(
            color.hue(),
            color.saturation(),
            lightness,
        ));
    }

    fn background_saturation(self: &Self) -> i32 {
        return self.background_color.hsl_saturation();
    }

    fn set_background_saturation(mut self: Pin<&mut Self>, saturation: i32) {
        let color = self.as_mut().background_color.to_owned();
        self.as_mut().set_background_color(QColor::from_hsl(
            color.hue(),
            saturation,
            color.lightness(),
        ));
    }

    fn font_size_label(self: &Self) -> QString {
        let qualities = self.contrast_qualities();
        let size = self.font_size().to_owned();

        let quality = if size >= 18 {
            qualities.large
        } else if size > 13 {
            qualities.medium
        } else {
            qualities.small
        };

        return match quality {
            Quality::Bad => {
                i18n(format!("Font size {size}px is bad with the current contrast").as_str())
            }
            Quality::Good => {
                i18n(format!("Font size {size}px is good with the current contrast").as_str())
            }
            Quality::Perfect => {
                i18n(format!("Font size {size}px is perfect with the current contrast").as_str())
            }
        };
    }

    fn contrast_qualities(self: &Self) -> ContrastQualities {
        match self.contrast() {
            it if it > 7.0 => ContrastQualities {
                small: Quality::Perfect,
                medium: Quality::Perfect,
                large: Quality::Perfect,
            },
            it if it > 4.5 => ContrastQualities {
                small: Quality::Good,
                medium: Quality::Good,
                large: Quality::Perfect,
            },
            it if it > 3.0 => ContrastQualities {
                small: Quality::Bad,
                medium: Quality::Bad,
                large: Quality::Good,
            },
            _ => ContrastQualities {
                small: Quality::Bad,
                medium: Quality::Bad,
                large: Quality::Bad,
            },
        }
    }

    fn display_text_color(self: &Self) -> QColor {
        if self.contrast() > 3.0 {
            return self.text_color.clone();
        }
        if luminosity(self.background_color()) > 0.5 {
            return QColor::from_rgb(0, 0, 0);
        }
        return QColor::from_rgb(255, 255, 255);
    }

    fn contrast(self: &Self) -> f32 {
        Self::contrast_for_colors(self.text_color(), self.background_color())
    }

    fn contrast_for_colors(color_foreground: &QColor, color_background: &QColor) -> f32 {
        let lum1 = luminosity(color_foreground);
        let lum2 = luminosity(color_background);

        if lum1 > lum2 {
            return (lum1 + 0.05) / (lum2 + 0.05);
        }

        return (lum2 + 0.05) / (lum1 + 0.05);
    }

    fn random(mut self: Pin<&mut Self>) {
        let mut rng = thread_rng();
        let mut col = || rng.gen_range(0..256);

        loop {
            let text_color = QColor::from_rgb(col(), col(), col());
            let bg_color = QColor::from_rgb(col(), col(), col());

            if Self::contrast_for_colors(&text_color, &bg_color) > 3.5 {
                self.as_mut().set_text_color(text_color);
                self.as_mut().set_background_color(bg_color);
                break;
            }
        }
    }

    fn reverse(mut self: Pin<&mut Self>) {
        let text_color = self.text_color.to_owned();
        let background_color = self.background_color.to_owned();
        self.as_mut().set_text_color(background_color);
        self.as_mut().set_background_color(text_color);
    }

    fn grab_color(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            match dbus::grab_color().await {
                Ok(vec) => {
                    let color = QColor::from_rgb_f(vec[0], vec[1], vec[2]);
                    qt_thread
                        .queue(move |mut qobj| {
                            qobj.as_mut().set_grabbed_color(color);
                        })
                        .unwrap();
                }
                Err(_) => {}
            };
        });
    }
}

impl cxx_qt::Initialize for ffi::Kontrast {
    fn initialize(mut self: core::pin::Pin<&mut Self>) {
        self.as_mut()
            .on_text_color_changed(|qobject| {
                qobject.contrast_changed();
            })
            .release();
        self.as_mut()
            .on_background_color_changed(|qobject| {
                qobject.contrast_changed();
            })
            .release();
        self.as_mut().random();
    }
}

fn luminosity(color: &QColor) -> f32 {
    let red = color.red_f();
    let green = color.green_f();
    let blue = color.blue_f();

    let red_lum = if red <= 0.03928 {
        red / 12.92
    } else {
        ((red + 0.055) / 1.055).powf(2.4)
    };
    let green_lum = if green <= 0.03928 {
        green / 12.92
    } else {
        ((green + 0.055) / 1.055).powf(2.4)
    };
    let blue_lum = if blue <= 0.03928 {
        blue / 12.92
    } else {
        ((blue + 0.055) / 1.055).powf(2.4)
    };

    return 0.2126 * red_lum + 0.7152 * green_lum + 0.0722 * blue_lum;
}
