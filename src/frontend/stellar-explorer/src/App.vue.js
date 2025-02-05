import Header from './components/Header.vue';
import SearchBar from './components/SearchBar.vue';
import MainTitle from './components/MainTitle.vue';
import StatisticsCards from './components/StatisticsCards.vue';
import BlockchainData from './components/BlockchainData.vue';
import Charts from './components/Charts.vue';
import LatestLedgers from './components/LatestLedgers.vue';
import Results from './components/Results.vue';
import Footer from './components/Footer.vue';
export default (await import('vue')).defineComponent({
    components: {
        Header,
        SearchBar,
        MainTitle,
        StatisticsCards,
        BlockchainData,
        Charts,
        LatestLedgers,
        Results,
        Footer,
    },
    data() {
        return {
            result: null,
        };
    },
    methods: {
        async handleSearch(query) {
            console.log('User searched for:', query);
            this.performSearch(query);
        },
        async performSearch(query) {
            // const query = document.getElementById('searchQuery').value
            if (!query) {
                alert('Please enter a query.');
                return;
            }
            const apiUrl = 'api/search';
            const data = { query };
            try {
                const response = await fetch(apiUrl, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
                });
                const result = await response.json();
                this.result = result;
                // if (!response.ok) {
                //   throw new Error(`HTTP error! status: ${response.status}`)
                // }
            }
            catch (error) {
                console.error('Error:', error);
                this.result = { detail: 'An error occurred while fetching the data.' };
            }
        },
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    const __VLS_componentsOption = {
        Header,
        SearchBar,
        MainTitle,
        StatisticsCards,
        BlockchainData,
        Charts,
        LatestLedgers,
        Results,
        Footer,
    };
    let __VLS_components;
    let __VLS_directives;
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        id: ("app"),
    });
    const __VLS_0 = {}.Header;
    /** @type { [typeof __VLS_components.Header, ] } */ ;
    // @ts-ignore
    const __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({}));
    const __VLS_2 = __VLS_1({}, ...__VLS_functionalComponentArgsRest(__VLS_1));
    const __VLS_6 = {}.SearchBar;
    /** @type { [typeof __VLS_components.SearchBar, ] } */ ;
    // @ts-ignore
    const __VLS_7 = __VLS_asFunctionalComponent(__VLS_6, new __VLS_6({
        ...{ 'onSearch': {} },
    }));
    const __VLS_8 = __VLS_7({
        ...{ 'onSearch': {} },
    }, ...__VLS_functionalComponentArgsRest(__VLS_7));
    let __VLS_12;
    const __VLS_13 = {
        onSearch: (__VLS_ctx.handleSearch)
    };
    let __VLS_9;
    let __VLS_10;
    var __VLS_11;
    const __VLS_14 = {}.Results;
    /** @type { [typeof __VLS_components.Results, ] } */ ;
    // @ts-ignore
    const __VLS_15 = __VLS_asFunctionalComponent(__VLS_14, new __VLS_14({
        result: ((__VLS_ctx.result)),
    }));
    const __VLS_16 = __VLS_15({
        result: ((__VLS_ctx.result)),
    }, ...__VLS_functionalComponentArgsRest(__VLS_15));
    const __VLS_20 = {}.MainTitle;
    /** @type { [typeof __VLS_components.MainTitle, ] } */ ;
    // @ts-ignore
    const __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, new __VLS_20({}));
    const __VLS_22 = __VLS_21({}, ...__VLS_functionalComponentArgsRest(__VLS_21));
    const __VLS_26 = {}.StatisticsCards;
    /** @type { [typeof __VLS_components.StatisticsCards, ] } */ ;
    // @ts-ignore
    const __VLS_27 = __VLS_asFunctionalComponent(__VLS_26, new __VLS_26({}));
    const __VLS_28 = __VLS_27({}, ...__VLS_functionalComponentArgsRest(__VLS_27));
    const __VLS_32 = {}.BlockchainData;
    /** @type { [typeof __VLS_components.BlockchainData, ] } */ ;
    // @ts-ignore
    const __VLS_33 = __VLS_asFunctionalComponent(__VLS_32, new __VLS_32({}));
    const __VLS_34 = __VLS_33({}, ...__VLS_functionalComponentArgsRest(__VLS_33));
    const __VLS_38 = {}.Charts;
    /** @type { [typeof __VLS_components.Charts, ] } */ ;
    // @ts-ignore
    const __VLS_39 = __VLS_asFunctionalComponent(__VLS_38, new __VLS_38({}));
    const __VLS_40 = __VLS_39({}, ...__VLS_functionalComponentArgsRest(__VLS_39));
    const __VLS_44 = {}.LatestLedgers;
    /** @type { [typeof __VLS_components.LatestLedgers, ] } */ ;
    // @ts-ignore
    const __VLS_45 = __VLS_asFunctionalComponent(__VLS_44, new __VLS_44({}));
    const __VLS_46 = __VLS_45({}, ...__VLS_functionalComponentArgsRest(__VLS_45));
    const __VLS_50 = {}.Footer;
    /** @type { [typeof __VLS_components.Footer, ] } */ ;
    // @ts-ignore
    const __VLS_51 = __VLS_asFunctionalComponent(__VLS_50, new __VLS_50({}));
    const __VLS_52 = __VLS_51({}, ...__VLS_functionalComponentArgsRest(__VLS_51));
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
