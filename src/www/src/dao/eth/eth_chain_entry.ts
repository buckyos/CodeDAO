interface ETHChainEntry {
    networkId?: number;
    chainId: number;
}

export const ETHChainEntryList: { [name: string]: ETHChainEntry } = {
    mainNet: {
        networkId: 1,
        chainId: 1
    },
    ropsten: {
        networkId: 3,
        chainId: 3
    },
    rinkeby: {
        networkId: 4,
        chainId: 4
    },
    // 水龙头: https://goerli-faucet.mudit.blog/
    goerli: {
        networkId: 5,
        chainId: 5
    },
    kovan: {
        networkId: 42,
        chainId: 42
    },
    ethClassic: {
        networkId: 1,
        chainId: 61
    },
    morden: {
        networkId: 2,
        chainId: 62
    },
    // 水龙头: https://sepoliafaucet.net/
    sepolia: {
        chainId: 11155111
    }
};
