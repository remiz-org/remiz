// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: 'Remiz',
    tagline: 'Build and deploy package, the easy way',
    url: 'https://remiz-org.github.io',
    baseUrl: '/remiz/',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    favicon: 'img/favicon.ico',
    organizationName: 'remiz', // Usually your GitHub org/user name.
    projectName: 'remiz', // Usually your repo name.

    presets: [
        [
            'classic',
            /** @type {import('@docusaurus/preset-classic').Options} */
            ({
                docs: {
                    sidebarPath: require.resolve('./sidebars.js'),
                    // Please change this to your repo.
                    editUrl: 'https://github.com/remiz-org/remiz/tree/main/docs/',
                },
                theme: {
                    customCss: require.resolve('./src/css/custom.css'),
                },
            }),
        ],
    ],

    themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
        ({
        navbar: {
            title: 'Remiz',
            logo: {
                alt: 'Remiz Logo',
                src: 'img/logo.svg',
            },
            items: [{
                    type: 'doc',
                    docId: 'intro',
                    position: 'left',
                    label: 'Docs',
                },
                {
                    type: 'localeDropdown',
                    position: 'right',
                },
                {
                    href: 'https://github.com/remiz-org/remiz',
                    label: 'GitHub',
                    position: 'right',
                },
            ],
        },
        footer: {
            style: 'dark',
            links: [{
                    title: 'Docs',
                    items: [{
                        label: 'Tutorial',
                        to: '/docs/intro',
                    }, ],
                },
                {
                    title: 'Community',
                    items: [{
                            label: 'Stack Overflow',
                            href: 'https://stackoverflow.com/',
                        },
                        {
                            label: 'Discord',
                            href: 'https://discordapp.com/',
                        },
                        {
                            label: 'Twitter',
                            href: 'https://twitter.com/',
                        },
                    ],
                },
                {
                    title: 'More',
                    items: [{
                        label: 'GitHub',
                        href: 'https://github.com/remiz-org/remiz',
                    }, ],
                },
            ],
            copyright: `Copyright © ${new Date().getFullYear()} Remiz, Inc.`,
        },
        prism: {
            theme: lightCodeTheme,
            darkTheme: darkCodeTheme,
        },
        algolia: {
            // L'ID de l'application fourni par Algolia
            appId: '70LG9BV9XD',

            // Clé d'API publique : il est possible de la committer en toute sécurité
            apiKey: 'a515066fb5e5a82d809634f72cf9e86c',

            indexName: 'remiz',

            // Facultatif : voir la section doc ci-dessous
            contextualSearch: true,

            // Facultatif : Spécifiez les domaines où la navigation doit se faire par window.location au lieu de history.push. Utile lorsque notre configuration Algolia explore plusieurs sites de documentation et que nous voulons naviguer vers eux avec window.location.href.
            // externalUrlRegex: 'external\\.com|domain\\.com',

            // Facultatif : paramètres de recherche de Algolia
            searchParameters: {},

            // Facultatif : chemin pour la page de recherche qui est activée par défaut (`false` pour le désactiver)
            searchPagePath: 'search',

            //... autres paramètres d'Algolia
        },
    }),
    i18n: {
        defaultLocale: 'en',
        locales: ['en', 'fr'],
    },
};

module.exports = config;