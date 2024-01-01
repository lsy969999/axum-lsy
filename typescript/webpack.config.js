const path = require('path');

module.exports = {
  entry: './src/index.ts',
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: '[name].[ext]',
              outputPath: '../image/gen', // 이미지를 저장할 폴더 경로
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'lsy.js',
    path: path.resolve(__dirname, '../assets/js'),
    library: 'lsy',
  },
};