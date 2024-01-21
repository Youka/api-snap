{{- define "app.name" -}}
{{- if eq .Release.Name .Chart.Name }}
{{- .Chart.Name }}
{{- else }}
{{- print .Chart.Name "-" .Release.Name }}
{{- end }}
{{- end }}

{{- define "app.labels" -}}
helm/chart: {{ print .Chart.Name "-" .Chart.Version }}
app.kubernetes.io/name: {{ .Chart.Name }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}
