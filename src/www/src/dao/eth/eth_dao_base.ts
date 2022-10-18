import Web3 from 'web3';
import * as cyfs from 'cyfs-sdk';
import detectEthereumProvider from '@metamask/detect-provider';
import { AbiItem } from 'web3-utils';
import { AbstractProvider } from 'web3-core';
import { Contract } from 'web3-eth-contract';
import { ETHChainEntryList } from './eth_chain_entry';

let web3: Web3 | undefined;

export async function checkWeb3(): Promise<cyfs.BuckyResult<Web3>> {
    if (web3) {
        return cyfs.Ok(web3);
    }

    const provider = await detectEthereumProvider<AbstractProvider>();
    if (!provider) {
        const msg = 'no eth provider, please install the plugin.';
        console.log(msg);
        return cyfs.Err(new cyfs.BuckyError(cyfs.BuckyErrorCode.ErrorState, msg));
    }

    if (provider!.request) {
        try {
            const r = await provider!.request({
                method: 'wallet_switchEthereumChain',
                params: [{ chainId: Web3.utils.numberToHex(ETHChainEntryList.sepolia.chainId) }]
            });

            console.log('wallet_switchEthereumChain result: ', r);

            if (r) {
                return cyfs.Err(
                    new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, 'switch chain failed')
                );
            }
        } catch (error) {
            return cyfs.Err(new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, 'switch chain failed'));
        }
    }

    if (!web3) {
        web3 = new Web3();
        web3.setProvider(provider);
    }

    return cyfs.Ok(web3);
}

// 查询ETH账号地址列表
export async function requestAccountList(): Promise<cyfs.BuckyResult<string[]>> {
    const r = await checkWeb3();
    if (r.err) {
        return r;
    }
    const accounts = await web3?.eth.requestAccounts();
    if (!accounts) {
        console.error(`no account: ${JSON.stringify(accounts)}`);
        return cyfs.Err(new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, 'no account'));
    }
    return cyfs.Ok(accounts);
}

export interface ETHdaoInitOption {
    ethAccount: string;
    name: string;
    symbol: string;
    cyfsOwner: cyfs.ObjectId;
}

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface ETHdaoContractInitParams {}

export interface ETHdaoContractDescription {
    template: () => string;
    bytesCode: () => string; // '0xHHHHHHH...'
    abi: (name: string) => AbiItem | undefined;
    allAbi: () => AbiItem[] | AbiItem;
    initParams: (params: ETHdaoContractInitParams) => Array<unknown> | undefined;
}

export class ETHdaoBuilder {
    public static async create(
        description: ETHdaoContractDescription,
        options: ETHdaoInitOption
    ): Promise<cyfs.BuckyResult<ETHdaoBuilder>> {
        const r = await checkWeb3();
        if (r.err) {
            return r;
        }
        return cyfs.Ok(new ETHdaoBuilder(description, options));
    }

    protected m_description: ETHdaoContractDescription;
    protected m_initOptions: ETHdaoInitOption;

    protected constructor(description: ETHdaoContractDescription, options: ETHdaoInitOption) {
        this.m_description = description;
        this.m_initOptions = { ...options };
    }

    public async deploy(
        gas: number,
        params: ETHdaoContractInitParams
    ): Promise<cyfs.BuckyResult<string>> {
        const contract = new web3!.eth.Contract(this.m_description.allAbi(), undefined, {
            from: this.m_initOptions.ethAccount,
            gas,
            data: this.m_description.bytesCode()
        });

        const args = this.m_description.initParams(params);
        if (!args) {
            console.error('invalid params');
            return cyfs.Err(
                new cyfs.BuckyError(cyfs.BuckyErrorCode.InvalidParam, 'invalid params')
            );
        }

        let contractAddress: string | undefined;
        let error: cyfs.BuckyError | undefined;

        const r = await contract
            .deploy({
                data: this.m_description.bytesCode(),
                arguments: [this.m_initOptions.name, this.m_initOptions.symbol, ...args]
            })
            .send({ from: this.m_initOptions.ethAccount, gas })
            .on('sending', () => {
                console.log('sending contract');
            })
            .on('sent', () => {
                console.log('sent contract');
            })
            .on('transactionHash', (receipt) => {
                console.log('transaction contract:', receipt);
            })
            .on('confirmation', (confNumber, receipt) => {
                console.log('transaction confirmed:', confNumber, receipt.transactionHash);
            })
            .on('receipt', (receipt) => {
                contractAddress = receipt.contractAddress;
            })
            .on('error', (err) => {
                const msg = `deploy contract(${this.m_description.template()}) failed: ${err}`;
                console.error(`deploy contract(${this.m_description.template()}) failed:`, err);
                error = new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, msg);
            });

        console.log('deploy contract done');

        if (!error) {
            if (contractAddress) {
                return cyfs.Ok(contractAddress!);
            } else {
                const msg = `deploy contract(${this.m_description.template()}) failed: no receipt?`;
                console.error(msg);
                return cyfs.Err(new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, msg));
            }
        } else {
            return cyfs.Err(error);
        }
    }

    public get web3(): Web3 {
        return web3!;
    }
}

export abstract class ETHdao {
    protected m_initOptions: ETHdaoInitOption;
    protected m_contractAddress: string;
    protected m_contract: Contract;

    protected constructor(contractAddress: string, contract: Contract, options: ETHdaoInitOption) {
        this.m_initOptions = { ...options };
        this.m_contractAddress = contractAddress;
        this.m_contract = contract;
    }

    public get contract(): Contract {
        return this.m_contract;
    }

    public get web3(): Web3 {
        return web3!;
    }

    public get contractAddress(): string {
        return this.m_contractAddress;
    }

    public get ethAccount(): string {
        return this.m_initOptions.ethAccount;
    }

    public get name(): string {
        return this.m_initOptions.name;
    }

    public get symbol(): string {
        return this.m_initOptions.symbol;
    }

    public get cyfsOwner(): cyfs.ObjectId {
        return this.m_initOptions.cyfsOwner;
    }

    public abstract get template(): string;
}
