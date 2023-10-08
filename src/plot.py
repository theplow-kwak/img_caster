import plotly.express as px
import pandas as pd
import argparse


def draw(df, filename):

    for col in ["start", "end"]:
        df[col] = pd.to_datetime(df[col])

    print(df['start'].min(), df['end'].max())

    fig = px.timeline(df, x_start="start", x_end="end",
                      y="io_type", color="latency", title=filename)

    # THIS IS A NECESSARY UPDATE!!!!
    fig.update_xaxes(range=[df['start'].min(), df['end'].max()])
    fig.update_traces(width=0.3)
    fig.update_layout(title_x=0.5,
                      title_y=0.9,
                      title_xanchor="center",
                      title_yanchor="middle",
                      height=400,
                      )

    fig.show()


def main():
    argparser = argparse.ArgumentParser()
    argparser.add_argument('filename', help='trace data file (csv)')
    argparser.add_argument('-v', '--verbose', nargs='?',
                           default='s', help='verbose display')
    args = argparser.parse_args()

    df = pd.read_csv(args.filename, skipinitialspace=True)

    draw(df, args.filename)


if __name__ == "__main__":
    main()
