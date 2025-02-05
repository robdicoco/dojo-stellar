import ApexCharts from 'vue3-apexcharts';
export default (await import('vue')).defineComponent({
    name: 'Charts',
    components: {
        apexchart: ApexCharts,
    },
    data() {
        return {
            // Time period tabs for each chart
            priceTabs: ['1D', '1W', '1M', '1Y'],
            operationsTabs: ['1D', '1W', '1M', '1Y'],
            transactionsTabs: ['1D', '1W', '1M', '1Y'],
            // Selected time periods
            selectedPriceTab: '1D',
            selectedOperationsTab: '1D',
            selectedTransactionsTab: '1D',
            // Price Chart Data
            priceChartOptions: {
                chart: {
                    toolbar: {
                        show: false,
                    },
                },
                xaxis: {
                    type: 'datetime',
                },
                yaxis: {
                    labels: {
                        formatter: function (val) {
                            return '$' + val.toFixed(2);
                        },
                    },
                },
            },
            priceSeries: [
                {
                    name: 'Price',
                    data: this.generateData(),
                },
            ],
            // Operations Chart Data
            operationsChartOptions: {
                chart: {
                    toolbar: {
                        show: false,
                    },
                },
                xaxis: {
                    type: 'datetime',
                },
            },
            operationsSeries: [
                {
                    name: 'Operations',
                    data: this.generateData(),
                },
            ],
            // Transactions Chart Data
            transactionsChartOptions: {
                chart: {
                    toolbar: {
                        show: false,
                    },
                },
                xaxis: {
                    type: 'datetime',
                },
            },
            transactionsSeries: [
                {
                    name: 'Transactions',
                    data: this.generateData(),
                },
            ],
        };
    },
    methods: {
        generateData() {
            const data = [];
            const now = new Date().getTime();
            for (let i = 0; i < 30; i++) {
                const timestamp = now - i * 24 * 60 * 60 * 1000;
                const value = Math.floor(Math.random() * 100);
                data.push([timestamp, value]);
            }
            return data;
        },
    },
}); /* PartiallyEnd: #3632/script.vue */
function __VLS_template() {
    const __VLS_ctx = {};
    const __VLS_componentsOption = {
        apexchart: ApexCharts,
    };
    let __VLS_components;
    let __VLS_directives;
    ['time-period-tabs', 'time-period-tabs', 'chart-container',];
    // CSS variable injection 
    // CSS variable injection end 
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("charts") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("chart-container") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.h2, __VLS_intrinsicElements.h2)({});
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("time-period-tabs") },
    });
    for (const [tab, index] of __VLS_getVForSourceType((__VLS_ctx.priceTabs))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.button, __VLS_intrinsicElements.button)({
            ...{ onClick: (...[$event]) => {
                    __VLS_ctx.selectedPriceTab = tab;
                } },
            key: ((index)),
            ...{ class: (({ active: __VLS_ctx.selectedPriceTab === tab })) },
        });
        (tab);
    }
    const __VLS_0 = {}.apexchart;
    /** @type { [typeof __VLS_components.Apexchart, typeof __VLS_components.apexchart, typeof __VLS_components.Apexchart, typeof __VLS_components.apexchart, ] } */ ;
    // @ts-ignore
    const __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({
        type: ("area"),
        height: ("350"),
        options: ((__VLS_ctx.priceChartOptions)),
        series: ((__VLS_ctx.priceSeries)),
    }));
    const __VLS_2 = __VLS_1({
        type: ("area"),
        height: ("350"),
        options: ((__VLS_ctx.priceChartOptions)),
        series: ((__VLS_ctx.priceSeries)),
    }, ...__VLS_functionalComponentArgsRest(__VLS_1));
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("chart-container") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.h2, __VLS_intrinsicElements.h2)({});
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("time-period-tabs") },
    });
    for (const [tab, index] of __VLS_getVForSourceType((__VLS_ctx.operationsTabs))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.button, __VLS_intrinsicElements.button)({
            ...{ onClick: (...[$event]) => {
                    __VLS_ctx.selectedOperationsTab = tab;
                } },
            key: ((index)),
            ...{ class: (({ active: __VLS_ctx.selectedOperationsTab === tab })) },
        });
        (tab);
    }
    const __VLS_6 = {}.apexchart;
    /** @type { [typeof __VLS_components.Apexchart, typeof __VLS_components.apexchart, typeof __VLS_components.Apexchart, typeof __VLS_components.apexchart, ] } */ ;
    // @ts-ignore
    const __VLS_7 = __VLS_asFunctionalComponent(__VLS_6, new __VLS_6({
        type: ("bar"),
        height: ("350"),
        options: ((__VLS_ctx.operationsChartOptions)),
        series: ((__VLS_ctx.operationsSeries)),
    }));
    const __VLS_8 = __VLS_7({
        type: ("bar"),
        height: ("350"),
        options: ((__VLS_ctx.operationsChartOptions)),
        series: ((__VLS_ctx.operationsSeries)),
    }, ...__VLS_functionalComponentArgsRest(__VLS_7));
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("chart-container") },
    });
    __VLS_elementAsFunction(__VLS_intrinsicElements.h2, __VLS_intrinsicElements.h2)({});
    __VLS_elementAsFunction(__VLS_intrinsicElements.div, __VLS_intrinsicElements.div)({
        ...{ class: ("time-period-tabs") },
    });
    for (const [tab, index] of __VLS_getVForSourceType((__VLS_ctx.transactionsTabs))) {
        __VLS_elementAsFunction(__VLS_intrinsicElements.button, __VLS_intrinsicElements.button)({
            ...{ onClick: (...[$event]) => {
                    __VLS_ctx.selectedTransactionsTab = tab;
                } },
            key: ((index)),
            ...{ class: (({ active: __VLS_ctx.selectedTransactionsTab === tab })) },
        });
        (tab);
    }
    const __VLS_12 = {}.apexchart;
    /** @type { [typeof __VLS_components.Apexchart, typeof __VLS_components.apexchart, typeof __VLS_components.Apexchart, typeof __VLS_components.apexchart, ] } */ ;
    // @ts-ignore
    const __VLS_13 = __VLS_asFunctionalComponent(__VLS_12, new __VLS_12({
        type: ("bar"),
        height: ("350"),
        options: ((__VLS_ctx.transactionsChartOptions)),
        series: ((__VLS_ctx.transactionsSeries)),
    }));
    const __VLS_14 = __VLS_13({
        type: ("bar"),
        height: ("350"),
        options: ((__VLS_ctx.transactionsChartOptions)),
        series: ((__VLS_ctx.transactionsSeries)),
    }, ...__VLS_functionalComponentArgsRest(__VLS_13));
    ['charts', 'chart-container', 'time-period-tabs', 'active', 'chart-container', 'time-period-tabs', 'active', 'chart-container', 'time-period-tabs', 'active',];
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
