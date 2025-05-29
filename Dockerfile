FROM chromadb/chroma:1.0.10

# uvicorn 설치
RUN pip install uvicorn

ENV CHROMA_HOST=0.0.0.0
ENV CHROMA_PORT=8000

EXPOSE 8000

# uvicorn 으로 직접 실행
CMD ["uvicorn", "chromadb.app:app", "--host", "0.0.0.0", "--port", "8000"]
