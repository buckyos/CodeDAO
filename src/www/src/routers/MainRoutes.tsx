import React from 'react';
import { Switch, Route, useHistory } from 'react-router-dom';
import { Spin } from 'antd';
import { requestLocal } from '@src/utils/index';
import { stack, stackInfo } from '@src/utils/stack';
import { userInfoAtom, UserInfoStore } from '@src/stores/user';
import { ResponseCheckUser } from '@src/types';
import { useRecoilState } from 'recoil';
import { mainRouters } from './main';

const AppRoutes: React.FC = () => {
    const history = useHistory();
    // check user
    const [userInfo, setUserInfo] = useRecoilState(userInfoAtom);
    React.useLayoutEffect(() => {
        (async function () {
            const r = await requestLocal<ResponseCheckUser>('user/checkInit', {
                owner: stackInfo.owner.to_base_58()
            });
            if (r.err || r.data === undefined) {
                console.error('requestLocal user/checkInit failed', r.msg);
                return;
            }
            if (!r.data.userInit) {
                history.push('/user/init');
                return;
            }
            if (r.data.user) {
                setUserInfo({
                    ...r.data.user,
                    id: stackInfo.owner.toString()
                });
                UserInfoStore.setData(r.data.user, stackInfo.owner.toString());
            }
        })();
    }, []);

    if (!userInfo.name) {
        return <Spin></Spin>;
    }

    return (
        <Switch>
            {mainRouters.map((route) => {
                return (
                    <Route exact={route.exact} path={route.path} key={route.path}>
                        {<route.component />}
                    </Route>
                );
            })}
        </Switch>
    );
};

export default AppRoutes;
