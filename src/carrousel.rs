use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ImageContainer = <View> {
        width: Fit,
        height: Fit,
        visible: false,
        image = <Image> {
            width: 400
            height: 266
        }
    }

    Carrousel = {{Carrousel}} {
        flow: Down,

        pages: [
            page1,
            page2,
            page3,
        ]

        page1 = <ImageContainer> {
            image = {
                source: dep("crate://self/resources/image1.png")
            }
        }
        page2 = <ImageContainer> {
            image = {
                source: dep("crate://self/resources/image2.png")
            }
        }
        page3 = <ImageContainer> {
            image = {
                source: dep("crate://self/resources/image3.png")
            }
        }
    }
}

#[derive(Live, LiveHook)]
#[live_ignore]
pub enum CarrouselPageOrder {
    #[pick] Normal,
    Reverse
}

#[derive(Live)]
pub struct Carrousel {
    #[deref]
    view: View,

    #[live]
    page_order: CarrouselPageOrder,

    #[rust(0)]
    current_page: u8,

    #[live]
    pages: Vec<LiveId>
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }

    fn after_apply(&mut self, _cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc()  {
            let current_page_live_id = self.pages[self.current_page as usize];
            self.view.view(&[current_page_live_id]).set_visible(true);
        }
    }
}

impl Widget for Carrousel {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => if fe.is_over {
                self.update_current_page();
                self.reset_frames_visibility();

                let current_page_live_id = self.pages[self.current_page as usize];
                self.view.view(&[current_page_live_id]).set_visible(true);

                self.redraw(cx);
            }
            _ => ()
        };
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.view.draw_walk_widget(cx, walk)
    }
}

impl Carrousel {
    fn reset_frames_visibility(&mut self) {
        for page_id in &self.pages {
            self.view.view(&[*page_id]).set_visible(false);
        }
    }

    fn update_current_page(&mut self) {
        match self.page_order {
            CarrouselPageOrder::Normal => {
                self.current_page = (self.current_page + 1) % self.pages.len() as u8;
            }
            CarrouselPageOrder::Reverse => {
                self.current_page = (self.current_page + self.pages.len() as u8 - 1) % self.pages.len() as u8;
            }
        }
    }
}
