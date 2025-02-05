export default (await import('vue')).defineComponent({
    name: 'SearchBar',
    data() {
        return {
            searchQuery: '', // Stores the current search input
            placeholderText: 'Search for labels, ledgers, contracts, transactions, operations, accounts, assets...', // Placeholder text
        };
    },
    methods: {
        handleSearch() {
            // Emit the search query to the parent component only if the query is not empty
            if (this.searchQuery.trim()) {
                this.$emit('search', this.searchQuery);
            }
        },
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    let __VLS_components;
    let __VLS_directives;
    ['search-bar', 'icon', 'icon',];
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("search-bar") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ onClick: (__VLS_ctx.handleSearch) },
        ...{ class: ("icon") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.i, __VLS_intrinsicElements.i)({
        ...{ class: ("fas fa-search") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.input)({
        ...{ onKeyup: (__VLS_ctx.handleSearch) },
        type: ("text"),
        value: ((__VLS_ctx.searchQuery)),
        placeholder: ((__VLS_ctx.placeholderText)),
    });
    ['search-bar', 'icon', 'fas', 'fa-search',];
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
