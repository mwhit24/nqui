import { defineStore } from "pinia";

interface ConnectionState {
    connections: Connection[];
    connectionString: string;
}
export interface Connection {
    connectionName: string;
    connectionString: string;
}

export const useConnectionStore = defineStore('connection', {
    state: (): ConnectionState => ({
        connectionString: '',
        connections: []
    }),
    getters: {
        getConnections: (state: any) => {
            return state.connections;
        },
        getConnectionString: (state: any) => {
            return state.connectionString;
        },
    },
    actions: {
        updateConnections(connection: Connection): void {
            this.connections.push(connection);
        },
       setConnectionString(connectionString: string): void {
           this.connectionString = connectionString;
       }
    }
})
