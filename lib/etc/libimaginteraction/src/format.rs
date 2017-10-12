//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use handlebars::{Handlebars, HelperDef, JsonRender, RenderError, RenderContext, Helper};
use ansi_term::Colour;
use ansi_term::Style;

#[derive(Clone, Copy)]
pub struct ColorizeBlackHelper;

impl HelperDef for ColorizeBlackHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Black, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizeBlueHelper;

impl HelperDef for ColorizeBlueHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Blue, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizeCyanHelper;

impl HelperDef for ColorizeCyanHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Cyan, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizeGreenHelper;

impl HelperDef for ColorizeGreenHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Green, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizePurpleHelper;

impl HelperDef for ColorizePurpleHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Purple, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizeRedHelper;

impl HelperDef for ColorizeRedHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Red, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizeWhiteHelper;

impl HelperDef for ColorizeWhiteHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::White, h, hb, rc)
    }
}

#[derive(Clone, Copy)]
pub struct ColorizeYellowHelper;

impl HelperDef for ColorizeYellowHelper {
    fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        colorize(Colour::Yellow, h, hb, rc)
    }
}

fn colorize(color: Colour, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));

    try!(write!(rc.writer(), "{}", color.paint(p.value().render())));
    Ok(())
}

#[derive(Clone, Copy)]
pub struct UnderlineHelper;

impl HelperDef for UnderlineHelper {
    fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
        RenderError> {
            let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
            let s = Style::new().underline();
            try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
            Ok(())
        }
}

#[derive(Clone, Copy)]
pub struct BoldHelper;

impl HelperDef for BoldHelper {
    fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
        RenderError> {
            let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
            let s = Style::new().bold();
            try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
            Ok(())
        }
}

#[derive(Clone, Copy)]
pub struct BlinkHelper;

impl HelperDef for BlinkHelper {
    fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
        RenderError> {
            let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
            let s = Style::new().blink();
            try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
            Ok(())
        }
}

#[derive(Clone, Copy)]
pub struct StrikethroughHelper;

impl HelperDef for StrikethroughHelper {
    fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
        RenderError> {
            let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
            let s = Style::new().strikethrough();
            try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
            Ok(())
        }
}

pub fn register_all_color_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("black"  , Box::new(ColorizeBlackHelper));
    handlebars.register_helper("blue"   , Box::new(ColorizeBlueHelper));
    handlebars.register_helper("cyan"   , Box::new(ColorizeCyanHelper));
    handlebars.register_helper("green"  , Box::new(ColorizeGreenHelper));
    handlebars.register_helper("purple" , Box::new(ColorizePurpleHelper));
    handlebars.register_helper("red"    , Box::new(ColorizeRedHelper));
    handlebars.register_helper("white"  , Box::new(ColorizeWhiteHelper));
    handlebars.register_helper("yellow" , Box::new(ColorizeYellowHelper));
}

pub fn register_all_format_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("underline"     , Box::new(UnderlineHelper));
    handlebars.register_helper("bold"          , Box::new(BoldHelper));
    handlebars.register_helper("blink"         , Box::new(BlinkHelper));
    handlebars.register_helper("strikethrough" , Box::new(StrikethroughHelper));
}

