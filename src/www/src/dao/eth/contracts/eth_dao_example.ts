import * as cyfs from 'cyfs-sdk';
import { AbiItem } from 'web3-utils';
import { ContractSendMethod } from 'web3-eth-contract';
import {
    checkWeb3,
    ETHdao,
    ETHdaoContractInitParams,
    ETHdaoContractDescription,
    ETHdaoInitOption
} from '../eth_dao_base';
import bytecode from './bytecode';
import abiArray from './abis';

const template = 'example';
const { abis: abiMap, constructor: contractConstructor } = mapAbi(abiArray);

function mapAbi(items: AbiItem[]): { abis: { [name: string]: AbiItem }; constructor: AbiItem } {
    let constructor: AbiItem | undefined;
    const abis: { [name: string]: AbiItem } = {};

    for (const abi of items) {
        if (abi.name) {
            abis[abi.name] = abi;
        } else {
            constructor = abi;
        }
    }

    console.assert(constructor);

    return {
        abis,
        constructor: constructor!
    };
}

export interface ExampleInitParams extends ETHdaoContractInitParams {
    initMembers: { [ethAccount: string]: cyfs.ObjectId }; // 初始成员列表
    tokenOwners: { [ethAccount: string]: number }; // token持有列表
}

class ExampleContractDescription implements ETHdaoContractDescription {
    public template(): string {
        return template;
    }

    public bytesCode(): string {
        return bytecode;
    }

    public abi(name: string): AbiItem | undefined {
        return abiMap[name];
    }

    public allAbi(): AbiItem | AbiItem[] {
        return abiArray;
    }

    public initParams(
        _params: ETHdaoContractInitParams
    ): [string[], number[], string[], Array<string>] | undefined {
        const params = _params as ExampleInitParams;

        const tokenOwners: string[] = [];
        const ownAmount: number[] = [];
        for (const [account, amount] of Object.entries(params.tokenOwners)) {
            tokenOwners.push(account);
            ownAmount.push(amount);
        }

        const memberAccounts: string[] = [];
        const memberDIDs: Array<string> = [];
        for (const [account, did] of Object.entries(params.initMembers)) {
            memberAccounts.push(account);
            memberDIDs.push(`0x${did.as_slice().toHex()}`);
        }

        if (tokenOwners.length === 0 || memberAccounts.length === 0) {
            return;
        }

        return [tokenOwners, ownAmount, memberAccounts, memberDIDs];
    }
}

export const exampleContractDescription = new ExampleContractDescription();

export interface ForumTopic {
    name: string;
    description: string;
    sec: number;
}

export enum ForumState {
    Active = 0,
    Passed = 1,
    Rejected = 2,
    Closed = 3
}

export interface VoteProgress {
    state: ForumState;
    pros: string[]; // 赞成者列表
    dissenters: string[]; // 反对者列表
}

export class ETHdaoExample extends ETHdao {
    public get template(): string {
        return template;
    }

    public static async create(
        contractAddress: string,
        options: ETHdaoInitOption
    ): Promise<cyfs.BuckyResult<ETHdaoExample>> {
        const r = await checkWeb3();
        if (r.err) {
            return r;
        }

        const web3 = r.unwrap();
        const contract = new web3.eth.Contract(abiArray, contractAddress, { data: bytecode });

        return cyfs.Ok(new ETHdaoExample(contractAddress, contract, options));
    }

    public async createForum(topic: ForumTopic): Promise<cyfs.BuckyResult<void>> {
        const method = this.contract.methods.createForum(
            topic.name,
            topic.description,
            topic.sec
        ) as ContractSendMethod;

        let result: cyfs.BuckyResult<void> = cyfs.Ok(undefined);
        await method.send({ from: this.ethAccount }).on('error', (err) => {
            const msg = `create forum(${topic.name}) failed, err: ${err}`;
            console.error(`create forum(${topic.name}) failed, err:`, err);
            result = cyfs.Err(new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, msg));
        });

        console.log(`contract create forum(${topic.name})`);

        return result;
    }

    public async getBalance(ethAccount: string): Promise<cyfs.BuckyResult<number>> {
        const method = this.contract.methods.balanceOf(ethAccount) as ContractSendMethod;

        const balance = await method.call({ from: this.ethAccount });

        console.log(`contract get balance(${ethAccount}): `, balance);
        if (balance || balance === 0) {
            return cyfs.Ok(balance);
        } else {
            return cyfs.Err(
                new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, `get balance(${ethAccount})`)
            );
        }
    }

    public async getDID(ethAccount: string): Promise<cyfs.BuckyResult<cyfs.ObjectId>> {
        const method = this.contract.methods.memberDIDs(ethAccount) as ContractSendMethod;

        const didHex = await method.call({ from: this.ethAccount });

        console.log(`contract get did(${ethAccount}): `, didHex);
        if (didHex) {
            const did = cyfs.ObjectId.copy_from_slice(Buffer.from(didHex, 'hex'));
            return cyfs.Ok(did);
        } else {
            return cyfs.Err(
                new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, `get did(${ethAccount})`)
            );
        }
    }

    public async vote(forumName: string, isAgree: boolean): Promise<cyfs.BuckyResult<void>> {
        const method = this.contract.methods.vote(forumName, isAgree) as ContractSendMethod;

        let result: cyfs.BuckyResult<void> = cyfs.Ok(undefined);
        await method.send({ from: this.ethAccount }).on('error', (err) => {
            const msg = `vote(${forumName}|${isAgree}) failed, err: ${err}`;
            console.error(`contract vote(${forumName}|${isAgree}) failed, err:`, err);
            result = cyfs.Err(new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, msg));
        });

        console.log(`contract vote(${forumName}|${isAgree}) done`);

        return result;
    }

    public async getForumState(forumName: string): Promise<cyfs.BuckyResult<VoteProgress>> {
        const method = this.contract.methods.getForumState(forumName) as ContractSendMethod;

        const state:
            | {
                  isClosed: boolean;
                  pros: string[];
                  dissenters: string[];
              }
            | undefined = await method.call({ from: this.ethAccount });

        console.log(`contract get forum state(${forumName}): `, state);

        if (state) {
            const prog: VoteProgress = {
                state: state.isClosed ? ForumState.Closed : ForumState.Active,
                pros: state.pros,
                dissenters: state.dissenters
            };
            return cyfs.Ok(prog);
        } else {
            return cyfs.Err(
                new cyfs.BuckyError(cyfs.BuckyErrorCode.Failed, `get forum state(${forumName})`)
            );
        }
    }
}
