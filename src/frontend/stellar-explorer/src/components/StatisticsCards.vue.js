export default (await import('vue')).defineComponent({
    name: 'StatisticsCards',
    data() {
        return {
            cards: [
                {
                    icon: 'fas fa-hashtag', // Rank icon
                    value: '13',
                    label: 'RANK',
                    percentage: '0',
                },
                {
                    icon: 'fas fa-dollar-sign', // Price icon
                    value: '$0.3933',
                    label: 'PRICE',
                    percentage: '0',
                },
                {
                    icon: 'fas fa-chart-line', // Market Cap icon
                    value: '$12,025,964,968',
                    label: 'MARKET CAP',
                    percentage: '0',
                },
                {
                    icon: 'fas fa-exchange-alt', // 24h Volume icon
                    value: '$268,861,566',
                    label: '24H VOLUME',
                    percentage: '0',
                },
            ],
        };
    },
    methods: {
        getPercentageClass(percentage) {
            if (percentage > 0)
                return 'positive';
            if (percentage < 0)
                return 'negative';
            return 'neutral';
        },
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    let __VLS_components;
    let __VLS_directives;
    ['card', 'card', 'card',];
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("statistics-cards") },
    });
    for (const [card, index] of __VLS_getVForSourceType((__VLS_ctx.cards))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
            ...{ class: ("card") },
            key: ((index)),
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
            ...{ class: ("icon") },
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.i, __VLS_intrinsicElements.i)({
            ...{ class: ((card.icon)) },
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
            ...{ class: ("data") },
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
            ...{ class: ("value") },
        });
        (card.value);
        __VLS_elementAsFunction(__VLS_intrinsicElements.p, __VLS_intrinsicElements.p)({
            ...{ class: ("label") },
        });
        (card.label);
        __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
            ...{ class: ("percentage") },
            ...{ class: ((__VLS_ctx.getPercentageClass(card.percentage))) },
        });
        __VLS_elementAsFunction(__VLS_intrinsicElements.span, __VLS_intrinsicElements.span)({});
        (card.percentage);
    }
    ['statistics-cards', 'card', 'icon', 'data', 'value', 'label', 'percentage',];
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
