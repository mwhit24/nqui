import {defineStore} from "pinia";

export type Tab = {
    title: string;
    table: Record<string, string>;
}

export const useLayoutStore = defineStore('layout', {
    state: () => ({
        currentTable: {} as Record<string, string>,
        tabs: [] as Tab[],
    }),
    getters: {
        getCurrentTable: (state) => {
            state.currentTable
        },
        getTabs: (state) => {
            state.tabs
        }
    },
    actions: {
        removeTab(tabIndex: number): void {
            this.tabs.splice(tabIndex, 1)
        },
        setCurrentTable(table: Record<string, string>): void {
            this.currentTable = table;
        },
        updateTabs(tab: Tab): void {
            const tabExists = !!this.tabs.find(x => x.title === tab.title);
            if (!tabExists) {
                this.tabs.push(tab);
            }
        }
    }
})
