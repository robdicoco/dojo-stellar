export default (await import('vue')).defineComponent({
    name: 'MainTitle',
    data() {
        return {
            title: 'StellarChain | Lumen Explorer', // Main title
            subtitle: 'StellarChain Explorer: Your Lumen Blockchain Discovery Tool', // Subtitle
        };
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    let __VLS_components;
    let __VLS_directives;
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.img)({
        alt: ("Vue logo"),
        ...{ class: ("logo") },
        src: ("@/assets/logo_explorer.png"),
        width: ("180"),
        height: ("180"),
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("main-title") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.h1, __VLS_intrinsicElements.h1)({
        ...{ class: ("title") },
    });
    (__VLS_ctx.title);
    __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
        ...{ class: ("subtitle") },
    });
    (__VLS_ctx.subtitle);
    ['logo', 'main-title', 'title', 'subtitle',];
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
