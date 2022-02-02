import React from 'react';
import clsx from 'clsx';
import styles from './HomepageFeatures.module.css';

const FeatureList = [
  {
    title: 'Easy to Use',
    Svg: require('../../static/img/terminal.svg').default,
    description: (
      <>
        Remiz was designed from the ground up to be easily installed and
        used to build and deploy your packages quickly.
      </>
    ),
  },
  {
    title: 'Extensible',
    Svg: require('../../static/img/toml.svg').default,
    description: (
      <>
        Write your configuration in <code>TOML</code> and use any
        programming language to build/deploy components of your projet.
      </>
    ),
  },
  {
    title: 'Powered by Rust',
    Svg: require('../../static/img/rustacean-flat-happy.svg').default,
    description: (
      <>
        Remiz is written in Rust. Remiz aims to be reliable, secure, fast.
        The best of all ? The project is open source (MIT licensed).
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} alt={title} />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
