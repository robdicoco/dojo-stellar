export default (await import('vue')).defineComponent({
    name: 'BlockchainData',
    data() {
        return {
            // Metrics for the left column
            leftMetrics: [
                { label: 'Total Accounts', value: '8,847,022,699' },
                { label: 'Total Assets', value: '336,371' },
                { label: 'XLM in Circulation', value: '30,576,001,864' },
                { label: 'TPS (24h)', value: '0' },
                { label: 'Average Ledger Time', value: '0.05' },
                { label: 'Protocol Version', value: '0' },
            ],
            // Metrics for the right column
            rightMetrics: [
                { label: 'Ledgers', value: '0' },
                { label: 'Transactions', value: '0' },
                { label: 'Operations', value: '0' },
                { label: 'Payments', value: '0' },
                { label: 'DEX Trades', value: '1,448,237' },
                { label: 'DEX Volume', value: '$3,565,379' },
            ],
        };
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    let __VLS_components;
    let __VLS_directives;
    ['metric', 'blockchain-data', 'column', 'left-column',];
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("blockchain-data") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("column left-column") },
    });
    for (const [metric, index] of __VLS_getVForSourceType((__VLS_ctx.leftMetrics))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
            ...{ class: ("metric") },
            key: ((index)),
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
            ...{ class: ("label") },
        });
        (metric.label);
        __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
            ...{ class: ("value") },
        });
        (metric.value);
    }
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("column right-column") },
    });
    for (const [metric, index] of __VLS_getVForSourceType((__VLS_ctx.rightMetrics))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
            ...{ class: ("metric") },
            key: ((index)),
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
            ...{ class: ("label") },
        });
        (metric.label);
        __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
            ...{ class: ("value") },
        });
        (metric.value);
    }
    ['blockchain-data', 'column', 'left-column', 'metric', 'label', 'value', 'column', 'right-column', 'metric', 'label', 'value',];
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
