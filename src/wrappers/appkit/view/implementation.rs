use super::*;
use crate::wrappers::appkit::new_class_name;
use objc2::__framework_prelude::{AnyClass, AnyObject, Bool, Sel};
use objc2::ffi::objc_disposeClassPair;
use objc2::runtime::ClassBuilder;
use objc2::{msg_send, sel, ClassType, ProtocolType};
use objc2_app_kit::{NSEvent, NSTextInputClient, NSView};
use objc2_foundation::{
    NSArray, NSAttributedString, NSAttributedStringKey, NSNotFound, NSPoint, NSRange,
    NSRangePointer, NSRect, NSUInteger,
};
use std::ffi::c_void;

/// # Safety
///
/// This class is going to be destroyed when its first instance gets deallocated.
///
/// The returned reference must NOT be used after that point.
pub unsafe fn create_view_class<V: ViewImpl>() -> &'static AnyClass {
    // Use unique class names so that there are no conflicts between different
    // instances. The class is deleted when the view is released. Previously,
    // the class was stored in a OnceCell after creation. This way, we didn't
    // have to recreate it each time a view was opened, but now we don't leave
    // any class definitions lying around when the plugin is closed.
    let class_name = new_class_name("BaseviewNSView_");

    let mut class = ClassBuilder::new(&class_name, NSView::class()).unwrap();

    // Some DAW hosts keep global computer-keyboard shortcuts active unless the embedded first
    // responder advertises itself as a native macOS text-input client. egui-baseview still handles
    // the actual text editing through key events; this conformance makes AppKit and the host treat
    // the focused plugin view as a text-entry target instead of a generic NSView.
    if let Some(protocol) = <dyn NSTextInputClient>::protocol() {
        class.add_protocol(protocol);
    }

    // SAFETY: All of these function signatures are correct
    unsafe {
        class.add_method(
            sel!(acceptsFirstResponder),
            property_yes as extern "C-unwind" fn(_, _) -> _,
        );
        class.add_method(
            sel!(becomeFirstResponder),
            become_first_responder::<V> as extern "C-unwind" fn(_, _) -> _,
        );
        class.add_method(
            sel!(resignFirstResponder),
            resign_first_responder::<V> as extern "C-unwind" fn(_, _) -> _,
        );
        class.add_method(sel!(isFlipped), property_yes as extern "C-unwind" fn(_, _) -> _);
        class.add_method(
            sel!(preservesContentInLiveResize),
            property_no as extern "C-unwind" fn(_, _) -> _,
        );
        class.add_method(
            sel!(acceptsFirstMouse:),
            accepts_first_mouse as extern "C-unwind" fn(_, _, _) -> _,
        );

        class.add_method(
            sel!(windowShouldClose:),
            window_should_close::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(sel!(dealloc), dealloc::<V> as extern "C-unwind" fn(_, _));
        class.add_method(
            sel!(viewWillMoveToWindow:),
            view_will_move_to_window::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(sel!(hitTest:), hit_test::<V> as extern "C-unwind" fn(_, _, _) -> _);
        class.add_method(
            sel!(updateTrackingAreas:),
            update_tracking_areas::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );

        class.add_method(sel!(mouseMoved:), mouse_moved::<V> as extern "C-unwind" fn(_, _, _) -> _);
        class.add_method(
            sel!(mouseDragged:),
            mouse_moved::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(rightMouseDragged:),
            mouse_moved::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(otherMouseDragged:),
            mouse_moved::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );

        class.add_method(
            sel!(scrollWheel:),
            scroll_wheel::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );

        class.add_method(
            sel!(viewDidChangeBackingProperties:),
            view_did_change_backing_properties::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );

        class.add_method(
            sel!(draggingEntered:),
            dragging_entered::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(prepareForDragOperation:),
            prepare_for_drag_operation::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(performDragOperation:),
            perform_drag_operation::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(draggingUpdated:),
            dragging_updated::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(draggingExited:),
            dragging_exited::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );
        class.add_method(
            sel!(handleNotification:),
            handle_notification::<V> as extern "C-unwind" fn(_, _, _) -> _,
        );

        class.add_method(sel!(mouseDown:), mouse_down::<V> as extern "C-unwind" fn(_, _, _));
        class.add_method(sel!(mouseUp:), mouse_up::<V> as extern "C-unwind" fn(_, _, _));
        class.add_method(
            sel!(rightMouseDown:),
            right_mouse_down::<V> as extern "C-unwind" fn(_, _, _),
        );
        class.add_method(sel!(rightMouseUp:), right_mouse_up::<V> as extern "C-unwind" fn(_, _, _));
        class.add_method(
            sel!(otherMouseDown:),
            other_mouse_down::<V> as extern "C-unwind" fn(_, _, _),
        );
        class.add_method(sel!(otherMouseUp:), other_mouse_up::<V> as extern "C-unwind" fn(_, _, _));

        class.add_method(sel!(mouseEntered:), mouse_entered::<V> as extern "C-unwind" fn(_, _, _));
        class.add_method(sel!(mouseExited:), mouse_exited::<V> as extern "C-unwind" fn(_, _, _));

        class.add_method(sel!(keyDown:), key_down::<V> as extern "C-unwind" fn(_, _, _));
        class.add_method(sel!(keyUp:), key_up::<V> as extern "C-unwind" fn(_, _, _));
        class.add_method(sel!(flagsChanged:), flags_changed::<V> as extern "C-unwind" fn(_, _, _));

        class.add_method(
            sel!(insertText:replacementRange:),
            insert_text as extern "C-unwind" fn(_, _, _, _),
        );
        class.add_method(
            sel!(doCommandBySelector:),
            do_command_by_selector as extern "C-unwind" fn(_, _, _),
        );
        class.add_method(
            sel!(setMarkedText:selectedRange:replacementRange:),
            set_marked_text as extern "C-unwind" fn(_, _, _, _, _),
        );
        class.add_method(sel!(unmarkText), unmark_text as extern "C-unwind" fn(_, _));
        class.add_method(sel!(selectedRange), selected_range as extern "C-unwind" fn(_, _) -> _);
        class.add_method(sel!(markedRange), marked_range as extern "C-unwind" fn(_, _) -> _);
        class.add_method(sel!(hasMarkedText), has_marked_text as extern "C-unwind" fn(_, _) -> _);
        class.add_method(
            sel!(attributedSubstringForProposedRange:actualRange:),
            attributed_substring_for_proposed_range as extern "C-unwind" fn(_, _, _, _) -> _,
        );
        class.add_method(
            sel!(validAttributesForMarkedText),
            valid_attributes_for_marked_text as extern "C-unwind" fn(_, _) -> _,
        );
        class.add_method(
            sel!(firstRectForCharacterRange:actualRange:),
            first_rect_for_character_range as extern "C-unwind" fn(_, _, _, _) -> _,
        );
        class.add_method(
            sel!(characterIndexForPoint:),
            character_index_for_point as extern "C-unwind" fn(_, _, _) -> _,
        );
    }

    class.add_ivar::<*mut c_void>(BASEVIEW_STATE_IVAR);

    class.register()
}

// These NSTextInputClient methods intentionally expose conservative empty state. egui-baseview
// owns the editable text model and receives translated key events through Baseview's normal event
// path, so maintaining a second AppKit-side selection/composition model here would desynchronize
// the two. This is sufficient for native text-input recognition; full IME composition can be wired
// through these methods separately if Baseview gains an IME event model.
extern "C-unwind" fn insert_text(
    _this: &NSView, _sel: Sel, _string: &AnyObject, _replacement_range: NSRange,
) {
}

extern "C-unwind" fn do_command_by_selector(_this: &NSView, _sel: Sel, _selector: Sel) {}

extern "C-unwind" fn set_marked_text(
    _this: &NSView, _sel: Sel, _string: &AnyObject, _selected_range: NSRange,
    _replacement_range: NSRange,
) {
}

extern "C-unwind" fn unmark_text(_this: &NSView, _sel: Sel) {}

extern "C-unwind" fn selected_range(_this: &NSView, _sel: Sel) -> NSRange {
    NSRange::new(0, 0)
}

extern "C-unwind" fn marked_range(_this: &NSView, _sel: Sel) -> NSRange {
    NSRange::new(NSNotFound as usize, 0)
}

extern "C-unwind" fn has_marked_text(_this: &NSView, _sel: Sel) -> Bool {
    Bool::NO
}

extern "C-unwind" fn attributed_substring_for_proposed_range(
    _this: &NSView, _sel: Sel, _range: NSRange, actual_range: NSRangePointer,
) -> *mut NSAttributedString {
    if !actual_range.is_null() {
        unsafe { actual_range.write(NSRange::new(NSNotFound as usize, 0)) };
    }
    core::ptr::null_mut()
}

extern "C-unwind" fn valid_attributes_for_marked_text(
    _this: &NSView, _sel: Sel,
) -> *mut NSArray<NSAttributedStringKey> {
    Retained::autorelease_return(NSArray::from_slice(&[]))
}

extern "C-unwind" fn first_rect_for_character_range(
    _this: &NSView, _sel: Sel, _range: NSRange, actual_range: NSRangePointer,
) -> NSRect {
    if !actual_range.is_null() {
        unsafe { actual_range.write(NSRange::new(NSNotFound as usize, 0)) };
    }
    NSRect::ZERO
}

extern "C-unwind" fn character_index_for_point(
    _this: &NSView, _sel: Sel, _point: NSPoint,
) -> NSUInteger {
    NSNotFound as usize
}

pub extern "C-unwind" fn dealloc<V: ViewImpl>(this: &mut AnyObject, _sel: Sel) {
    let class = this.class();
    View::<V>::free_inner(this, class);

    if let Some(superclass) = class.superclass() {
        let () = unsafe { msg_send![super(this, superclass), dealloc] };
    }

    // SAFETY: This is safe as long as nobody holds a reference to this class.
    // On the Baseview side, this is enforced by the safety contract in `create_view_class`
    unsafe { objc_disposeClassPair(class as *const _ as *mut _) }
}

extern "C-unwind" fn property_yes(_this: &NSView, _sel: Sel) -> Bool {
    Bool::YES
}

extern "C-unwind" fn property_no(_this: &NSView, _sel: Sel) -> Bool {
    Bool::NO
}

extern "C-unwind" fn accepts_first_mouse(_this: &NSView, _sel: Sel, _event: &NSEvent) -> Bool {
    Bool::YES
}

extern "C-unwind" fn become_first_responder<V: ViewImpl>(this: &View<V>, _sel: Sel) -> Bool {
    V::become_first_responder(this.inner_ref()).into()
}

extern "C-unwind" fn resign_first_responder<V: ViewImpl>(this: &View<V>, _sel: Sel) -> Bool {
    V::resign_first_responder(this.inner_ref()).into()
}

extern "C-unwind" fn window_should_close<V: ViewImpl>(
    this: &View<V>, _: Sel, _sender: &AnyObject,
) -> Bool {
    V::window_should_close(this.inner_ref()).into()
}

extern "C-unwind" fn view_did_change_backing_properties<V: ViewImpl>(
    this: &View<V>, _: Sel, _: &AnyObject,
) {
    V::view_did_change_backing_properties(this.inner_ref());
}

extern "C-unwind" fn hit_test<V: ViewImpl>(
    this: &View<V>, _sel: Sel, point: NSPoint,
) -> Option<&NSView> {
    V::hit_test(this.inner_ref(), point)
}

extern "C-unwind" fn view_will_move_to_window<V: ViewImpl>(
    this: &View<V>, _self: Sel, new_window: Option<&NSWindow>,
) {
    V::view_will_move_to_window(this.inner_ref(), new_window);
}

extern "C-unwind" fn update_tracking_areas<V: ViewImpl>(this: &View<V>, _self: Sel, _: &AnyObject) {
    V::update_tracking_areas(this.inner_ref());
}

extern "C-unwind" fn mouse_moved<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::mouse_moved(this.inner_ref(), event);
}

extern "C-unwind" fn scroll_wheel<V: ViewImpl>(this: &View<V>, _: Sel, event: &NSEvent) {
    V::scroll_wheel(this.inner_ref(), event);
}

extern "C-unwind" fn dragging_entered<V: ViewImpl>(
    this: &View<V>, _sel: Sel, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>,
) -> NSDragOperation {
    V::dragging_entered(this.inner_ref(), sender)
}

extern "C-unwind" fn dragging_updated<V: ViewImpl>(
    this: &View<V>, _sel: Sel, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>,
) -> NSDragOperation {
    V::dragging_updated(this.inner_ref(), sender)
}

extern "C-unwind" fn prepare_for_drag_operation<V: ViewImpl>(
    this: &View<V>, _sel: Sel, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>,
) -> Bool {
    V::prepare_for_drag_operation(this.inner_ref(), sender).into()
}

extern "C-unwind" fn perform_drag_operation<V: ViewImpl>(
    this: &View<V>, _sel: Sel, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>,
) -> Bool {
    V::perform_drag_operation(this.inner_ref(), sender).into()
}

extern "C-unwind" fn dragging_exited<V: ViewImpl>(
    this: &View<V>, _sel: Sel, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>,
) {
    V::dragging_exited(this.inner_ref(), sender)
}

extern "C-unwind" fn handle_notification<V: ViewImpl>(
    this: &View<V>, _cmd: Sel, notification: &NSNotification,
) {
    V::handle_notification(this.inner_ref(), notification)
}

extern "C-unwind" fn mouse_entered<V: ViewImpl>(this: &View<V>, _: Sel, _: &AnyObject) {
    V::mouse_entered(this.inner_ref());
}

extern "C-unwind" fn mouse_exited<V: ViewImpl>(this: &View<V>, _: Sel, _: &AnyObject) {
    V::mouse_exited(this.inner_ref());
}

extern "C-unwind" fn key_down<V: ViewImpl>(this: &View<V>, _: Sel, event: &NSEvent) {
    V::key_down(this.inner_ref(), event);
}

extern "C-unwind" fn key_up<V: ViewImpl>(this: &View<V>, _: Sel, event: &NSEvent) {
    V::key_up(this.inner_ref(), event);
}

extern "C-unwind" fn flags_changed<V: ViewImpl>(this: &View<V>, _: Sel, event: &NSEvent) {
    V::flags_changed(this.inner_ref(), event);
}

extern "C-unwind" fn mouse_down<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::mouse_down(this.inner_ref(), event);
}

extern "C-unwind" fn mouse_up<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::mouse_up(this.inner_ref(), event);
}

extern "C-unwind" fn right_mouse_down<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::right_mouse_down(this.inner_ref(), event);
}

extern "C-unwind" fn right_mouse_up<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::right_mouse_up(this.inner_ref(), event);
}

extern "C-unwind" fn other_mouse_down<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::other_mouse_down(this.inner_ref(), event);
}

extern "C-unwind" fn other_mouse_up<V: ViewImpl>(this: &View<V>, _sel: Sel, event: &NSEvent) {
    V::other_mouse_up(this.inner_ref(), event);
}
