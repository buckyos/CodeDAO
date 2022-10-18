import React from 'react';
import { withRouter, Switch, Route, RouteComponentProps, Link } from 'react-router-dom';
import styles from './DaoIndex.css';
import Logo2Icon from '@src/assets/images/logo2.png';
import BannerHandIcon from '@src/assets/images/banner_hand.png';
import Banner2Icon from '@src/assets/images/banner2.png';

const DaoIndex: React.FC = () => {
    return (
        <div>
            <div className={styles.header1}>
                <div className={styles.header}>
                    <img className={styles.logo} src={Logo2Icon} />
                    <div className={styles.links}>
                        <Link className={styles.link} to={'/dao'}>
                            what’s CodeDao
                        </Link>
                        <Link className={styles.link} to={'/'}>
                            GIT
                        </Link>
                    </div>
                </div>
            </div>

            <div className={styles.banner}>
                <div className={styles.banner2}></div>

                <div className={styles.banner3}>
                    <img className={styles.hand} src={BannerHandIcon} />

                    <div className={styles.words}>
                        <div className={styles.bigword}>
                            Decentralized autonomous <br />
                            organizations (DAOs)
                        </div>
                        <div className={styles.smallword}>
                            <span></span>
                            Member-owned communities without centralized leadership.
                        </div>
                        <div className={styles.smallword}>
                            <span></span>A safe way to collaborate with internet strangers.
                        </div>
                        <div className={styles.smallword}>
                            <span></span>A safe place to commit funds to a specific cause.
                        </div>
                    </div>

                    <img className={styles.right} src={Banner2Icon} />

                    <div className={styles.circle2}></div>
                    <div className={styles.circle1}></div>
                    <div className={styles.circle3}></div>
                    <div className={styles.circle4}></div>
                    <div className={styles.circle5}></div>
                    <div className={styles.circle6}></div>
                    <div className={styles.circle7}></div>
                    <div className={styles.circle8}></div>
                </div>
                <div
                    className={styles.banner4}
                    style={{ backgroundImage: 'url(./img/background-shape.png)' }}
                ></div>
            </div>

            <div className={styles.out}>
                <div className={styles.circle11}></div>
                <div className={styles.circle12}></div>
                <div className={styles.circle13}></div>
                <div className={styles.circle14}></div>
                <div className={styles.circle15}></div>
                <div className={styles.circle16}></div>

                <div className={styles.content}>
                    <h1 className={styles.title}>What are DAOs?</h1>
                    <div className={styles.phase}>
                        DAOs are effective and safe way to work with like-minded folks around the
                        globe.
                    </div>

                    <h1 className={styles.title}>
                        Why do we need DAO to revive open source community
                    </h1>
                    <div className={styles.phase}>
                        DAO is a more advanced form of organization,more democratic,open and
                        transparent,
                        <br />
                        and can protect the internets of all parties(investors,core
                        team,contributors,product users).
                        <br />
                        It is well suitable for open source projects to motivate people to do open
                        source projects and collaborate globally.
                    </div>

                    <h1 className={styles.title}>Why we need CodeDao</h1>
                    <div className={styles.phase}>
                        CodeDao is a perfect tool for you to run your project DAOs.
                        <br />
                        It is a web3 service,fully decentralized deployed with no centralized
                        bankground servers.
                        <br />
                        People run their own clients and interact with each other peer-to-peer.
                        <br />
                        In CodeDao,you really own your accounts,Git repositories,tokens,so that you
                        can really run a project as a DAO.
                    </div>

                    <div className={styles.last}>
                        CodeDao’s DAO coming soon … but you can directly use git repository hosting
                    </div>
                </div>
            </div>
        </div>
    );
};

export default DaoIndex;
