# templates/_helpers.tpl

{{- define "rust-backend.name" -}}
{{ .Chart.Name }}
{{- end -}}

{{- define "rust-backend.fullname" -}}
{{ .Release.Name }}-{{ include "rust-backend.name" . }}
{{- end -}}
