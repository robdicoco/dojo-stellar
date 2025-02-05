import BlockSearch from './components/BlockSearch.vue';
import TransactionSearch from './components/TransactionSearch.vue';
import BalanceSearch from './components/BalanceSearch.vue';
export default (await import('vue')).defineComponent({
    name: 'App',
    components: {
        BlockSearch,
        TransactionSearch,
        BalanceSearch,
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    const __VLS_componentsOption = {
        BlockSearch,
        TransactionSearch,
        BalanceSearch,
    };
    let __VLS_components;
    let __VLS_directives;
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.header, __VLS_intrinsicElements.header)({});
    __VLS_elementAsFunction(__VLS_intrinsicElements.img)({
        alt: ("Lumen logo"),
        ...{ class: ("logo") },
        src: ("@/assets/logo2.png"),
        width: ("125"),
        height: ("125"),
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        id: ("app"),
    });
    const __VLS_0 = {}.BlockSearch;
    /** @type { [typeof __VLS_components.BlockSearch, ] } */ ;
    // @ts-ignore
    const __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({}));
    const __VLS_2 = __VLS_1({}, ...__VLS_functionalComponentArgsRest(__VLS_1));
    const __VLS_6 = {}.TransactionSearch;
    /** @type { [typeof __VLS_components.TransactionSearch, ] } */ ;
    // @ts-ignore
    const __VLS_7 = __VLS_asFunctionalComponent(__VLS_6, new __VLS_6({}));
    const __VLS_8 = __VLS_7({}, ...__VLS_functionalComponentArgsRest(__VLS_7));
    const __VLS_12 = {}.BalanceSearch;
    /** @type { [typeof __VLS_components.BalanceSearch, ] } */ ;
    // @ts-ignore
    const __VLS_13 = __VLS_asFunctionalComponent(__VLS_12, new __VLS_12({}));
    const __VLS_14 = __VLS_13({}, ...__VLS_functionalComponentArgsRest(__VLS_13));
    ['logo',];
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
