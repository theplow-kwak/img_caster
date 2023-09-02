import plotly.express as px

data = [
    {"Task": "Task A", "Start": "2023-08-01", "End": "2023-08-10"},
    {"Task": "Task B", "Start": "2023-08-15", "End": "2023-08-25"},
    {"Task": "Task C", "Start": "2023-09-05", "End": "2023-09-15"},
    # 다른 작업 추가 가능
]

fig = px.timeline(data, x_start="Start", x_end="End", y="Task", title="Broken Bar Chart")
fig.update_yaxes(categoryorder="total ascending")  # y 축의 카테고리 순서 설정
fig.show()
