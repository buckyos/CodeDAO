import React from 'react';
import ReactDOM from 'react-dom';
import { HashRouter as Router, Switch, Route, Redirect } from 'react-router-dom';
import { RecoilRoot } from 'recoil';
import Header from './components/Header/Header';
import MainRoutes from './routers/MainRoutes';
import InitUser from './pages/InitUser/InitUser';
import DaoIndex from './pages/Dao/DaoIndex';
import { stack, stackInfo } from '@src/utils/stack';

import './i18n/i18n';
import { initWithNativeStack } from './utils/cyfs_helper/stack_wraper';
import * as MetaClient from './utils/cyfs_helper/meta_client';
import { ENV_TARGET } from './utils/cyfs_helper/constant';

/* eslint-disable */
console.log = (console as any).origin.log;

declare const __VERSION__: String;
console.log('CYFS-git current version: ', __VERSION__);

const Main = () => {
    return (
        <Router>
            <Switch>
                <Route path="/user/init">
                    <InitUser />
                </Route>
                <Route path="/dao" exact>
                    <DaoIndex />
                </Route>
                <Route path="/">
                    <div className="main">
                        <Header />
                        <MainRoutes />
                    </div>
                </Route>
            </Switch>
        </Router>
    );
};

async function main() {
    await stack.online();
    await stackInfo.init();
    // TODO: 重构初始化Stack及api封装
    MetaClient.init(ENV_TARGET);
    await initWithNativeStack(stack);

    ReactDOM.render(
        <React.StrictMode>
            <RecoilRoot>
                <Main />
            </RecoilRoot>
        </React.StrictMode>,
        document.getElementById('main')
    );
}

main();
