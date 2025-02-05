export default (await import('vue')).defineComponent({
    data() {
        return {
            links: [
                { name: 'robdicoco', url: 'https://github.com/robdicoco' },
                { name: 'alfatektecnologia', url: 'https://github.com/alfatektecnologia' },
                { name: 'lucenfort', url: 'https://github.com/lucenfort' },
                { name: 'uederson-ferreira', url: 'https://github.com/uederson-ferreira' },
            ],
        };
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    let __VLS_components;
    let __VLS_directives;
    ['footer-link', 'source-code', 'source-code',];
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.footer, __VLS_intrinsicElements.footer)({
        ...{ class: ("footer") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("footer-content") },
    });
    for (const [link, index] of __VLS_getVForSourceType((__VLS_ctx.links))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.a, __VLS_intrinsicElements.a)({
            key: ((index)),
            href: ((link.url)),
            target: ("_blank"),
            ...{ class: ("footer-link") },
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.sub, __VLS_intrinsicElements.sub)({});
        __VLS_elementAsFunction(__VLS_intrinsicElements.b, __VLS_intrinsicElements.b)({});
        (link.name);
    }
    __VLS_elementAsFunction(__VLS_intrinsicElements.span, __VLS_intrinsicElements.span)({
        ...{ class: ("separator") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.span, __VLS_intrinsicElements.span)({
        ...{ class: ("source-code") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.a, __VLS_intrinsicElements.a)({
        href: ("https://github.com/robdicoco/dojo-stellar"),
        target: ("_blank"),
    });
    ['footer', 'footer-content', 'footer-link', 'separator', 'source-code',];
    var __VLS_slots;
    var $slots;
    let __VLS_inheritedAttrs;
    var $attrs;
    const __VLS_refs = {};
    var $refs;
    var $el;
    return {
        attrs: {},
        slots: __VLS_slots,
        refs: $refs,
        rootEl: $el,
    };
}
;
let __VLS_self;
